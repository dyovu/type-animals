// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn set_dynamic_libraries_path() {
    use std::env;
    use std::path::PathBuf;
    
    // 実行ファイルの場所を取得
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // ライブラリパスを構築
            let lib_dir = PathBuf::from(exe_dir).join("libs").join("macos");
            
            // パスが存在することを確認
            if lib_dir.exists() {
                // 環境変数を設定
                let dyld_path = env::var("DYLD_LIBRARY_PATH").unwrap_or_default();
                let new_path = format!("{}:{}", lib_dir.display(), dyld_path);
                env::set_var("DYLD_LIBRARY_PATH", new_path);
            }
        }
    }
}

fn main() {
    // ライブラリのパスを設定、SDLのバイナリをプロジェクト内に入れてそれを実行するため
    // set_dynamic_libraries_path();

    type_animals_lib::run()
}
