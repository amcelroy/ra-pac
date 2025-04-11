fn main() {
    #[cfg(feature = "rt")]
    {
        let device_name = match env::vars()
            .map(|(a, _)| a)
            .filter(|x| x.starts_with("CARGO_FEATURE_RA"))
            .collect::<Vec<_>>()
            .as_slice()
        {
            [feature] => feature.strip_prefix("CARGO_FEATURE_")
                .unwrap()
                .to_ascii_lowercase()
                .replace('_', "-"),
            [] => panic!("No RA MCU Cargo feature enabled"),
            _ => panic!("Multiple RA MCU Cargo features enabled"),
        };

        println!(
            "cargo:rustc-link-search={}/src/pacs/{}",
            env::var("CARGO_MANIFEST_DIR").unwrap(),
            device_name,
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
}
