fn main() {
    #[cfg(not(target_os = "macos"))]
    {
        println!("cargo:rustc-cfg=not_macos");
    }

    tauri_build::build()
}
