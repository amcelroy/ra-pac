use std::env;

fn main() {
    let enabled_features: Vec<String> = env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_RA"))
        .collect();

    if enabled_features.is_empty() {
        println!("cargo:warning=No RA MCU feature enabled. Proceeding without any RA MCU feature.");
    } else if enabled_features.len() == 1 {
        let device_name = enabled_features[0]
            .strip_prefix("CARGO_FEATURE_")
            .unwrap()
            .to_ascii_lowercase()
            .replace('_', "-");

        println!(
            "cargo:rustc-link-search={}/src/pacs/{}",
            env::var("CARGO_MANIFEST_DIR").unwrap(),
            device_name,
        );
    } else {
        panic!("Multiple RA MCU Cargo features enabled");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
