use std::env;
use std::fs;
use std::path::Path;

fn main() {
  println!("cargo::warning=Copy Assets Folder");

  // 1. 檢查 `assets` 來源資料夾是否存在
  if !Path::new("assets").exists() {
    // 如果不存在，就沒什麼好複製的，直接返回
    return;
  }

  // 2. 告訴 Cargo 如果 `assets` 資料夾的內容有變更，需要重新執行此腳本
  println!("cargo:rerun-if-changed=assets");

  // 3. 取得輸出目錄的路徑 (例如 target/debug 或 target/release)
  let out_dir = env::var("OUT_DIR").unwrap();
  let target_dir = Path::new(&out_dir)
    .ancestors().nth(3).unwrap().to_path_buf();

  // 4. 設定目標路徑
  let dest_path = target_dir.join("assets");

  fs::create_dir_all(&dest_path).unwrap();

  // 5. 執行複製
  copy_dir_all(Path::new("assets"), &dest_path).expect("Failed to copy assets files");
}

// 遞迴複製資料夾的輔助函式
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
  fs::create_dir_all(dst)?;
  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let ty = entry.file_type()?;
    if ty.is_dir() {
      copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
    } else {
      fs::copy(entry.path(), dst.join(entry.file_name()))?;
    }
  }
  Ok(())
}
