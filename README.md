
# RA-PAC

This repository contains the Peripheral Access Crate (PAC) for Renesas RA Microcontrollers.

The crate is automatically generated from the SVD files in [packs](https://www.keil.arm.com/packs) using [svd2pac](https://github.com/Infineon/svd2pac).

It serves as the foundation for the Hardware Abstraction Layer (HAL) in Rust for Renesas RA Series Microcontrollers.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
ra-pac = { version = "0.0.0", features = ["ra4m1", "rt"] }
```

## Target Architectures

Each device corresponds to a specific architecture target:

- **RA0, RA2 Series**: `thumbv8m.base-none-eabi` (Cortex-M23)

- **RA4, RA6 Series**:
  - Cortex-M4 devices: `thumbv7em-none-eabihf`
    - `ra4m1` (Cortex-M4)
    - `ra4m2` (Cortex-M4)
    - `ra4m3` (Cortex-M4)
    - `ra4w1` (Cortex-M4)
    - `ra6m1` (Cortex-M4)
    - `ra6m2` (Cortex-M4)
    - `ra6m3` (Cortex-M4)
    - `ra6t1` (Cortex-M4)
  - Cortex-M33 devices: `thumbv8m.main-none-eabihf`
    - The other RA4 devices

- **RA8 Series**: `thumbv8m.main-none-eabihf` (Cortex-M33)

## Supported Devices

- RA0E1
- RA2A1
- RA2A2
- RA2E1
- RA2E2
- RA2E3
- RA2L1
- RA4E1
- RA4E2
- RA4L1
- RA4M1
- RA4M2
- RA4M3
- RA4T1
- RA4W1
- RA6E1
- RA6E2
- RA6M1
- RA6M2
- RA6M3
- RA6M4
- RA6M5
- RA6T1
- RA6T2
- RA6T3
- RA8D1
- RA8E1
- RA8E2
- RA8M1

## License

This crate is licensed under either the MIT License or the Apache License, Version 2.0.

The contents of this crate are auto-generated and licensed under the following proprietary terms of Renesas Electronics Corporation:
This software is supplied by Renesas Electronics Corporation and is only intended for use with Renesas products. No other uses are authorized. This software is owned by Renesas Electronics Corporation and is protected under all applicable laws, including copyright laws.

This software is provided "AS IS" without warranties of any kind, and Renesas makes no warranties regarding this software, including but not limited to warranties of merchantability, fitness for a particular purpose, and non-infringement.

Renesas reserves the right, without notice, to make changes to this software and to discontinue the availability of this software. By using this software, you agree to the additional terms and conditions found at:
[Renesas Disclaimer](http://www.renesas.com/disclaimer)
