use glob::glob;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dest_path = PathBuf::from("assets");

    fs::create_dir_all(&dest_path).expect("Failed to create asset directory");

    println!("cargo:rerun-if-changed=../content/maneuvers/videos/");

    let pattern = "../../../content/maneuvers/videos/*.mp4";

    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(source) => {
                if source.is_file() {
                    let file_name = source.file_name().unwrap();
                    let destination = dest_path.join(file_name);

                    fs::copy(&source, &destination).expect("Failed to copy asset");

                    println!("cargo:rerun-if-changed={}", source.display());
                }
            }
            Err(e) => println!("cargo:warning=Error reading glob entry: {:?}", e),
        }
    }
}
