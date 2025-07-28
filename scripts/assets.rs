use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define source and destination paths
    let source_dir = Path::new("assets/fav");
    let release_dir = Path::new("target/dx/hmziq-xyz/release/web/public/assets/fav");

    // Create the destination directory structure if it doesn't exist
    fs::create_dir_all(&release_dir)?;

    // Copy all files from source to destination
    copy_dir_contents(&source_dir, &release_dir)?;

    println!("Successfully copied favicon assets to release directory");
    Ok(())
}

fn copy_dir_contents(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure destination exists
    fs::create_dir_all(dst)?;

    // Iterate through source directory
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dest_path = dst.join(&file_name);

        if path.is_dir() {
            // Recursively copy subdirectories
            copy_dir_contents(&path, &dest_path)?;
        } else {
            // Copy file
            fs::copy(&path, &dest_path)?;
            println!("Copied: {} -> {}", path.display(), dest_path.display());
        }
    }

    Ok(())
}
