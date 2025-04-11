# RA-PAC

This repository contains the Peripheral Access Crate (PAC) for Renesas RA Microcontrollers.

The crate is automatically generated from the SVD files in [ra_dfp-renesas](https://www.keil.arm.com/packs/ra_dfp-renesas/versions/) using [chiptool](https://github.com/embassy-rs/chiptool/).

It serves as the foundation for the Hardware Abstraction Layer (HAL) for Renesas RA Series Microcontrollers.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
ra-pac = { version = "0.1.0", features = ["ra6m3", "rt"] }
```

Choose the appropriate feature for your target device. **You must specify exactly one device feature.**

### Example Application

```rust
use ra_pac::ra6m3; // Import the device-specific module

fn main() {
  let peripherals = ra6m3::Peripherals::take().unwrap();

  // Access device peripherals
  let gpio = &peripherals.GPIO;
  // ...
}
```

## Features

### Device Selection Features

Select **exactly one** feature corresponding to your target device:

#### RA0 Series (Cortex-M23)

- `ra0e1`

#### RA2 Series (Cortex-M23)

- `ra2a1`
- `ra2a2`
- `ra2e1`
- `ra2e2`
- `ra2e3`
- `ra2l1`

#### RA4 Series (Cortex-M4 and Cortex-M33)

- `ra4e1` (Cortex-M33)
- `ra4m1` (Cortex-M4)
- `ra4m2` (Cortex-M4)
- `ra4m3` (Cortex-M4)
- `ra4w1` (Cortex-M4)

#### RA6 Series (Cortex-M4 and Cortex-M33)

- `ra6e1` (Cortex-M33)
- `ra6m1` (Cortex-M4)
- `ra6m2` (Cortex-M4)
- `ra6m3` (Cortex-M4)
- `ra6m4` (Cortex-M33)
- `ra6m5` (Cortex-M33)
- `ra6t1` (Cortex-M4)

### Additional Features

- `rt` - Enable Cortex-M runtime support.
- `defmt` - Enable `defmt` logging support.

## Target Architectures

Each device corresponds to a specific architecture target:

- **RA0, RA2 Series**: `thumbv8m.base-none-eabi` (Cortex-M23)
- **RA4, RA6 Series**:
  - Cortex-M4 devices: `thumbv7em-none-eabihf`
  - Cortex-M33 devices: `thumbv8m.main-none-eabihf`

## Supported Devices

- RA0E1
- RA2A1
- RA2A2
- RA2E1
- RA2E2
- RA2E3
- RA2L1
- RA4E1
- RA4M1
- RA4M2
- RA4M3
- RA4W1
- RA6E1
- RA6M1
- RA6M2
- RA6M3
- RA6M4
- RA6M5
- RA6T1

## License

The contents of this crate are auto-generated and licensed under the same terms as the underlying SVD file.
