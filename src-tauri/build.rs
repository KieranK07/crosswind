fn main() {
    // files embedded with include_dir! (macOS BepInEx, the ROUNDS layer)
    println!("cargo:rerun-if-changed=resources");

    println!(
        "cargo:rustc-env=BUILD_TIME={}",
        chrono::Utc::now().to_rfc3339()
    );

    tauri_build::build();
}
