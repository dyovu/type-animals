fn main() {
    // macOS用の設定
    println!("cargo:rustc-link-search=native=./libs/macos");
    
    // SDL2ライブラリのリンク
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_image");
    
    // 変更があった場合に再実行するための設定
    println!("cargo:rerun-if-changed=build.rs");
    
    tauri_build::build()
}
