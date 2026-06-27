use std::path::Path;

fn main() {
    let content_dir = Path::new("content");
    println!("cargo:rerun-if-changed=content");
    if content_dir.is_dir()
        && let Ok(entries) = std::fs::read_dir(content_dir)
    {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "md") {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}
