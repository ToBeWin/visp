fn main() {
    #[cfg(target_os = "macos")]
    {
        let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
        cc::Build::new()
            .file("src/macos_speech.m")
            .flag("-fobjc-arc")
            .flag("-fblocks")
            .compile("visp_macos_speech");

        println!("cargo:rustc-link-search=native={out_dir}");
        println!("cargo:rustc-link-lib=static=visp_macos_speech");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=Speech");
        println!("cargo:rustc-link-lib=framework=AVFoundation");
        println!("cargo:rerun-if-changed=src/macos_speech.m");
    }

    tauri_build::build()
}
