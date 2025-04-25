use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

fn main() {
    create_permission_files();
    tauri_build::build()
}

fn create_permission_files() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(out_dir);
    println!(">>>> OUT_DIR: {}", out_dir.display());

    for (key, value) in std::env::vars_os() {
        let key = key.to_string_lossy();
        // println!("{}: {}", key, value.to_string_lossy());

        if key.starts_with("UNTAR-") {
            let path = std::path::PathBuf::from(value);
            println!(">>>>>> {}", path.display());

            let tar_gz = File::open(path).unwrap();
            let tar = GzDecoder::new(tar_gz);

            let mut archive = Archive::new(tar);

            for entry in archive.entries().unwrap() {
                // Make sure there wasn't an I/O error
                let mut entry = entry.unwrap();

                entry.unpack_in(&out_dir).unwrap();

                // if tar::EntryType::Regular == entry.header().entry_type() {
                //     println!(">>>>>> - {}", entry.path().unwrap().display());
                // }
            }
        }
    }
}
