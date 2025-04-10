use std::env;
use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;

//const CARGO_FEATURE_PREFIX: &str = "CARGO_FEATURE_RA";
const CACHE_FILE: &str = "target/supported_devices_cache.txt"; // Cache location

// Function to get the list of supported devices from README.md
fn get_supported_devices() -> Vec<String> {
    let cache_file = Path::new(CACHE_FILE);

    // Check if the cache file exists and is recent enough
    if cache_file.exists() && cache_file.metadata().unwrap().modified().unwrap().elapsed().unwrap().as_secs() < 60 {
        // Use cached data
        let cached_data = fs::read_to_string(cache_file).expect("Failed to read cache file");
        return cached_data.lines().map(|line| line.trim().to_string()).collect();
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let readme_path = manifest_dir.join("README.md");

    let readme_content = fs::read_to_string(&readme_path).expect("Failed to read README.md");

    let mut devices = Vec::new();
    let mut in_section = false;

    for line in readme_content.lines() {
        if line.trim() == "## Supported devices" {
            in_section = true;
            continue;
        }

        if in_section {
            if line.trim().starts_with("- ") {
                let device = line.trim()[2..].trim().to_uppercase();
                devices.push(device);
            } else if line.trim().starts_with("#") {
                break;
            }
        }
    }

    if devices.is_empty() {
        panic!("No supported devices found in README.md");
    }

    // Cache the devices for the next build
    fs::write(cache_file, devices.join("\n")).expect("Failed to write cache file");

    devices
}

// Function to run cargo build for a single device feature
fn run_build_for_device(device: &str, is_release: bool) {
    let feature = format!("ra{}", device);
    let mut command = Command::new("cargo");

    command.arg("build")
        .arg(format!("--features={}", feature));

    // If release is specified, add the `--release` flag
    if is_release {
        command.arg("--release");
    }

    let output = command.output().expect("Failed to execute cargo build");

    if !output.status.success() {
        eprintln!("Error running cargo build for {}: {}", feature, String::from_utf8_lossy(&output.stderr));
    } else {
        eprintln!("Successfully built feature: {}", feature);
    }
}

// Function to run cargo build for all supported devices
fn run_build_for_all_devices(is_release: bool) {
    let supported_devices = get_supported_devices();
    eprintln!("Supported devices: {:?}", supported_devices); // Using eprintln! to ensure visibility
    for device in supported_devices {
        eprintln!("Building for device: {}", device); // Using eprintln! to ensure visibility
        // Convert device name to lowercase for the feature name
        let device = device.to_lowercase();
        run_build_for_device(&device, is_release);
    }
}

fn main() {
    // Check if any specific feature is passed
    let args: Vec<String> = env::args().collect();
    let is_release = args.contains(&"--release".to_string());

    if args.iter().any(|arg| arg.starts_with("--features=")) {
        // If a specific feature is provided (e.g., cargo build --features ra4m1)
        let feature = args.iter()
            .find(|arg| arg.starts_with("--features="))
            .expect("No feature provided")
            .split('=')
            .nth(1)
            .expect("Failed to parse feature");

        // Run the build only for the provided feature
        eprintln!("Building for specific feature: {}", feature); // Ensure visibility with eprintln!
        run_build_for_device(feature, is_release);
    } else {
        // Otherwise, run the build for all devices
        eprintln!("Building for all devices..."); // Ensure visibility with eprintln!
        run_build_for_all_devices(is_release);
    }

    // Ensure that if build.rs or README.md changes, the build script will rerun
    eprintln!("cargo:rerun-if-changed=build.rs");
    eprintln!("cargo:rerun-if-changed=README.md");
}
