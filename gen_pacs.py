#!/usr/bin/env python3

import os
import sys
import shutil
import subprocess
import glob
import tempfile
import re
import xml.etree.ElementTree as ET
from pathlib import Path
from collections import defaultdict, Counter

# Global constants should be in uppercase
RUST_BACKTRACE = "1"
RUST_FULLTRACE = "full"
RUST_LOG = "info"

def check_command_exists(command):
    """Check if a command exists in the system path."""
    if shutil.which(command) is None:
        print(f"{command} could not be found. Install it with the following command:")
        print("")
        if command == "chiptool":
            print("    cargo install --git https://github.com/embassy-rs/chiptool --locked")
        elif command == "svd":
            print("    pip install svdtools")
        print("")
        sys.exit(1)

def run_command(command, check=True):
    """Run a shell command and check for errors."""
    try:
        return subprocess.run(command, check=check, shell=True, text=True,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    except subprocess.CalledProcessError as e:
        print(f"Command failed: {e.cmd}")
        print(f"Output: {e.stdout}")
        print(f"Error: {e.stderr}")
        if check:
            sys.exit(1)
        return e

def analyze_svd_for_duplicates(root):
    """
    Analyze SVD file for potential name conflicts that could cause chiptool to fail.
    Returns a dictionary with information about duplicate names.
    """
    print("Analyzing SVD file for potential duplicate names...")

    # Check for duplicate peripheral names
    peripheral_names = [p.find('name').text for p in root.findall('.//peripheral/name/..')]
    duplicate_peripherals = [name for name, count in Counter(peripheral_names).items() if count > 1]

    # Check for registers with same name after transformation (removing non-alphanumeric chars)
    register_names_by_peripheral = defaultdict(list)
    transformed_names_by_peripheral = defaultdict(list)

    for peripheral in root.findall('.//peripheral'):
        peripheral_name = peripheral.find('name').text

        for register in peripheral.findall('.//register'):
            reg_name = register.find('name')
            if reg_name is not None and reg_name.text:
                original_name = reg_name.text
                # Simulate chiptool's transformation (as best we can guess)
                transformed_name = re.sub(r'[^a-zA-Z0-9_]', '_', original_name)
                register_names_by_peripheral[peripheral_name].append((original_name, transformed_name))
                transformed_names_by_peripheral[peripheral_name].append(transformed_name)

    # Find duplicates after transformation
    duplicate_transformed_registers = {}
    for peripheral, names in transformed_names_by_peripheral.items():
        duplicate_transformed_registers[peripheral] = [name for name, count in Counter(names).items() if count > 1]

    # Remove empty entries
    duplicate_transformed_registers = {k: v for k, v in duplicate_transformed_registers.items() if v}

    # Check for duplicate field names within registers
    duplicate_fields_by_register = {}

    for peripheral in root.findall('.//peripheral'):
        peripheral_name = peripheral.find('name').text

        for register in peripheral.findall('.//register'):
            reg_name = register.find('name')
            if reg_name is not None and reg_name.text:
                register_name = reg_name.text
                field_names = []

                for field in register.findall('.//field'):
                    field_name_elem = field.find('name')
                    if field_name_elem is not None and field_name_elem.text:
                        field_names.append(field_name_elem.text)

                # Find duplicates
                duplicate_fields = [name for name, count in Counter(field_names).items() if count > 1]
                if duplicate_fields:
                    key = f"{peripheral_name}.{register_name}"
                    duplicate_fields_by_register[key] = duplicate_fields

    return {
        'duplicate_peripherals': duplicate_peripherals,
        'duplicate_transformed_registers': duplicate_transformed_registers,
        'register_names_by_peripheral': register_names_by_peripheral,
        'duplicate_fields_by_register': duplicate_fields_by_register
    }

def fix_duplicate_names(root, analysis_results):
    """
    Fix duplicate names in the SVD file based on analysis results.
    """
    print("Fixing duplicate names in SVD file...")

    for peripheral_name, duplicate_names in analysis_results['duplicate_transformed_registers'].items():
        print(f"  Fixing duplicate names in peripheral '{peripheral_name}': {', '.join(duplicate_names)}")

        for dup_name in duplicate_names:
            # Find all registers that transform to this duplicate name
            registers_to_fix = []
            for peripheral in root.findall(f".//peripheral[name='{peripheral_name}']"):
                for register in peripheral.findall('.//register'):
                    reg_name = register.find('name')
                    if reg_name is not None and reg_name.text:
                        transformed = re.sub(r'[^a-zA-Z0-9_]', '_', reg_name.text)
                        if transformed == dup_name:
                            registers_to_fix.append((register, reg_name.text))

            # Add suffixes to make names unique
            for i, (register, original_name) in enumerate(registers_to_fix[1:], 1):
                new_name = f"{original_name}_{i}"
                print(f"    Renaming '{original_name}' to '{new_name}'")
                register.find('name').text = new_name

def fix_duplicate_field_names(root, analysis_results):
    """
    Fix duplicate field names in the SVD file by appending lsb and msb values
    to make them unique. Particularly handles 'Reserved' fields by always renaming
    them to Reserved_<lsb>_<msb> regardless of duplication.
    """
    print("Fixing duplicate field names in registers...")

    # Track all fields that need to be renamed
    fields_to_rename = []

    # First, handle duplicate fields identified during analysis
    duplicate_fields = analysis_results.get('duplicate_fields_by_register', {})
    for reg_key, field_names in duplicate_fields.items():
        peripheral_name, register_name = reg_key.split('.')
        print(f"  Fixing duplicate field names in {reg_key}: {', '.join(field_names)}")

        for field_name in field_names:
            for peripheral in root.findall(f".//peripheral[name='{peripheral_name}']"):
                for register in peripheral.findall(f".//register[name='{register_name}']"):
                    # Collect all fields with this name to be renamed
                    for field in register.findall(f".//field[name='{field_name}']"):
                        lsb_elem = field.find('lsb')
                        msb_elem = field.find('msb')

                        if lsb_elem is not None and lsb_elem.text and msb_elem is not None and msb_elem.text:
                            lsb = lsb_elem.text
                            msb = msb_elem.text
                            fields_to_rename.append((field, field_name, lsb, msb))

    # Additionally, find and rename all reserved fields regardless of duplication
    for peripheral in root.findall('.//peripheral'):
        peripheral_name = peripheral.find('name').text

        for register in peripheral.findall('.//register'):
            register_name = register.find('name').text

            for field in register.findall(".//field"):
                field_name_elem = field.find('name')
                if field_name_elem is not None and field_name_elem.text:
                    field_name = field_name_elem.text

                    # Check if this is a reserved field
                    if field_name.lower() == "reserved":
                        lsb_elem = field.find('lsb')
                        msb_elem = field.find('msb')

                        if lsb_elem is not None and lsb_elem.text and msb_elem is not None and msb_elem.text:
                            lsb = lsb_elem.text
                            msb = msb_elem.text
                            fields_to_rename.append((field, field_name, lsb, msb))

    # Now rename all the collected fields
    for field, original_name, lsb, msb in fields_to_rename:
        # For reserved fields, always use the lsb_msb format
        if original_name.lower() == "reserved":
            new_name = f"Reserved_{lsb}_{msb}"
        else:
            new_name = f"{original_name}_{lsb}_{msb}"

        name_elem = field.find('name')
        print(f"    Renaming '{name_elem.text}' to '{new_name}'")
        name_elem.text = new_name

def preprocess_svd_file(svd_file_path):
    """
    Preprocess SVD file to fix issues that might cause problems with chiptool.
    Returns the path to the preprocessed file.
    """
    # Create a temporary directory for preprocessed files
    temp_dir = tempfile.mkdtemp()

    try:
        # Parse the SVD file
        tree = ET.parse(svd_file_path)
        root = tree.getroot()

        # Fix 1: Fix access types (read,write -> read-write)
        for access_elem in root.findall(".//*[@access]"):
            if access_elem.get("access") == "read,write":
                access_elem.set("access", "read-write")

        # Fix 2: Remove derived_from attributes from registers
        for derived in root.findall(".//register[@derivedFrom]"):
            derived.attrib.pop('derivedFrom')

        # Fix 3: Add missing name tags in enumerated values
        for enum_value in root.findall(".//enumeratedValue"):
            if enum_value.find("name") is None:
                # Create a name tag with the value as content
                value_elem = enum_value.find("value")
                if value_elem is not None and value_elem.text:
                    name_elem = ET.SubElement(enum_value, "name")
                    name_elem.text = f"VALUE_{value_elem.text}"
                    # Move the name element to the beginning of the enumerated value
                    enum_value.remove(name_elem)
                    enum_value.insert(0, name_elem)

        # Fix 4: Remove all enumerated values completely
        for parent in root.findall(".//enumeratedValues"):
            for enum_value in list(parent.findall("./enumeratedValue")):
                parent.remove(enum_value)

        # Fix 5: Fix enumerated values with # prefix
        for value_elem in root.findall(".//value"):
            if value_elem.text and value_elem.text.startswith('#'):
                value_elem.text = value_elem.text[1:]

        # Fix 6: Fix ranges in descriptions (e.g., "A-B" to "A,B")
        for desc_elem in root.findall(".//description"):
            if desc_elem.text:
                desc_elem.text = re.sub(r'([A-Za-z])-([A-Za-z])', r'\1,\2', desc_elem.text)

        # Fix 7: Analyze SVD file for duplicates
        analysis_results = analyze_svd_for_duplicates(root)

        # Fix 8: Rename duplicate register names
        if analysis_results['duplicate_transformed_registers']:
            print("Found duplicate register names after transformation:")
            for peripheral, duplicates in analysis_results['duplicate_transformed_registers'].items():
                print(f"  Peripheral '{peripheral}': {', '.join(duplicates)}")
            fix_duplicate_names(root, analysis_results)

        # Fix 9: Rename duplicate field names in registers
        if analysis_results['duplicate_fields_by_register']:
            print("Found duplicate field names in registers:")
            for reg_key, duplicates in analysis_results['duplicate_fields_by_register'].items():
                print(f"  Register '{reg_key}': {', '.join(duplicates)}")
            fix_duplicate_field_names(root, analysis_results)

        # Get the filename
        filename = os.path.basename(svd_file_path)

        # Write the modified XML to a temporary file
        temp_file_path = os.path.join(temp_dir, filename)
        tree.write(temp_file_path, encoding='utf-8', xml_declaration=True)

        return temp_file_path, temp_dir

    except Exception as e:
        print(f"Error preprocessing XML in {svd_file_path}: {str(e)}")
        sys.exit(1)

def process_device(device_name):
    """Process a single device SVD file."""
    svd_file = f"svd/{device_name}.svd"

    # Check if the SVD file exists
    if not os.path.exists(svd_file):
        print(f"SVD file {svd_file} not found.")
        return False

    print(f"\n===== Processing device: {device_name} =====")

    # Preprocess the SVD file
    print(f"Preprocessing SVD file {device_name}...")
    preprocessed_svd, temp_dir = preprocess_svd_file(svd_file)

    # Ensure svd_patch directory exists
    os.makedirs("svd_patch", exist_ok=True)

    # Copy the preprocessed SVD file to the svd_patch directory
    dst = os.path.join("svd_patch", f"{device_name}.svd")
    try:
        shutil.copyfile(preprocessed_svd, dst)
        print(f"Copied {preprocessed_svd} to {dst}")
    except Exception as e:
        print(f"Error copying preprocessed SVD file: {str(e)}")
        return False

    try:
        # Generate files using chiptool with the preprocessed file
        print(f"Running chiptool for {device_name}...")
        result = run_command(f"chiptool generate --svd {preprocessed_svd}", check=False)

        if result.returncode != 0:
            print(f"Warning: chiptool failed for {device_name}")
            print(f"Output: {result.stdout}")
            print(f"Error: {result.stderr}")

            # Save the problematic SVD file for debugging
            debug_dir = Path("debug_svd")
            debug_dir.mkdir(exist_ok=True)
            debug_file = debug_dir / f"{device_name}_debug.svd"
            shutil.copyfile(preprocessed_svd, debug_file)
            print(f"Saved problematic SVD file to {debug_file} for debugging")

            # If there's a patched SVD file available, try using that
            patched_svd = os.path.join("svd_patch", f"{device_name.lower()}.svd.patched")
            if os.path.exists(patched_svd):
                print(f"Found patched SVD file: {patched_svd}")
                print(f"Attempting to run chiptool with the patched file...")
                result = run_command(f"chiptool generate --svd {patched_svd}", check=False)

                if result.returncode != 0:
                    print(f"Warning: chiptool also failed with the patched SVD file")
                    print(f"Output: {result.stdout}")
                    print(f"Error: {result.stderr}")
                    return False
                else:
                    print(f"Successfully generated files using patched SVD file")
            else:
                print(f"No patched SVD file found for {device_name}")
                print(f"You may need to manually create a patched SVD file at: {patched_svd}")
                return False

        # Format lib.rs
        run_command("rustfmt lib.rs")

        # Remove #![no_std] line
        with open("lib.rs", "r") as file:
            content = file.read()

        with open("lib.rs", "w") as file:
            file.write(content.replace('#![no_std]', ''))

        # Create directories and move files
        folder_name = device_name[0].lower() + device_name[3:7].lower()
        chip_dir = Path(f"src/pacs/{folder_name}")
        chip_dir.mkdir(parents=True, exist_ok=True)

        if os.path.exists("lib.rs"):
            shutil.move("lib.rs", f"{chip_dir}/pac.rs")

        if os.path.exists("device.x"):
            shutil.move("device.x", f"{chip_dir}/device.x")

        # Check and generate docs for the chip
        if chip_dir.exists():
            run_command(f"cargo check --features {device_name.lower()}", check=False)
            run_command(f"cargo doc --features {device_name.lower()}", check=False)

        return True

    finally:
        # Clean up temporary directory
        if temp_dir and os.path.exists(temp_dir):
            shutil.rmtree(temp_dir)

def main():
    # Check if required commands are installed
    check_command_exists("chiptool")
    check_command_exists("svd")

    # Create directories if they don't exist
    os.makedirs("src", exist_ok=True)
    os.makedirs("svd_patch", exist_ok=True)

    # Set environment variables
    os.environ["RUST_BACKTRACE"] = RUST_FULLTRACE
    os.environ["RUST_LOG"] = RUST_LOG

    # Check if a device name is provided as a command-line argument
    if len(sys.argv) > 1:
        device_name = sys.argv[1]
        success = process_device(device_name)
        if not success:
            print(f"Failed to process device {device_name}")
            sys.exit(1)
    else:
        # Process all SVD files in the svd directory
        print("No device name provided. Processing all SVD files in svd folder...")
        svd_files = glob.glob("svd/*.svd")

        if not svd_files:
            print("No SVD files found in svd directory.")
            sys.exit(1)

        successes = 0
        failures = 0

        for svd_file in svd_files:
            # Extract device name from filename
            device_name = os.path.basename(svd_file).split('.')[0]
            if process_device(device_name):
                successes += 1
            else:
                failures += 1

        print(f"\nProcessed all SVD files. Successes: {successes}, Failures: {failures}")

        if failures > 0:
            print(f"Warning: {failures} device(s) failed to process properly.")

    # Format code
    run_command("cargo fmt", check=False)
    print("Done.")

if __name__ == "__main__":
    main()
