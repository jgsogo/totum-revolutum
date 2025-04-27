use anyhow::{anyhow, Result};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::Write;
use tar::Archive;

fn main() -> Result<()> {
    create_permission_files()?;
    tauri_build::build();
    Ok(())
}

fn create_permission_files() -> Result<()> {
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = std::path::PathBuf::from(out_dir);
    // println!(">>>> OUT_DIR: {}", out_dir.display());

    for (key, value) in std::env::vars_os() {
        let key = key.to_string_lossy();
        // println!("{}: {}", key, value.to_string_lossy());

        if let Some(actual_env_var_name) = key.strip_prefix("UNTAR-") {
            // This envvar points to a zipped file containing the permission files
            // for some Tauri plugin (or the core one). We need to unpack these
            // files in some unique folder (inside `out_dir`) and create a file
            // providing the paths to all those files.
            //
            // Then, we add this to the environment variables so `tauri_build`
            // can collect them.
            let plugin_dir = {
                let plugin_folder_name = actual_env_var_name
                    .strip_suffix("_PERMISSION_FILES_PATH")
                    .unwrap_or(actual_env_var_name);
                let plugin_folder_name = plugin_folder_name.replace(":", "-");
                let plugin_dir = out_dir.join(plugin_folder_name.to_lowercase());
                std::fs::create_dir_all(&plugin_dir)?;
                plugin_dir
            };
            let mut all_files: Vec<std::path::PathBuf> = Vec::new();
            // - unpack everything into a directory
            // println!(">>>>>> {} -> {}", actual_env_var_name, plugin_dir.display());

            let path = std::path::PathBuf::from(value);
            let tar_gz = File::open(path)?;
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            for entry in archive.entries()? {
                let mut entry = entry?;
                if tar::EntryType::Regular == entry.header().entry_type() {
                    entry.unpack_in(&plugin_dir)?;
                    all_files.push(std::fs::canonicalize(plugin_dir.join(entry.path()?))?);
                }
            }

            // println!(">>>>>> {:?}", all_files);

            let plugin_permission_file_path = out_dir.join(format!("{}.json", plugin_dir.display()));
            let mut plugin_permission_file = File::create(&plugin_permission_file_path)?;
            let json = serde_json::to_string(&all_files)?;
            plugin_permission_file.write_all(json.as_bytes())?;

            println!(
                ">>>> {}: {}",
                actual_env_var_name,
                plugin_permission_file_path.display()
            );
            std::env::set_var(actual_env_var_name, plugin_permission_file_path);
        }

        // if key.starts_with("UNTAR-") {
        //     let path = std::path::PathBuf::from(value);
        //     // println!(">>>>>> {}", path.display());

        //     let tar_gz = File::open(path)?;
        //     let tar = GzDecoder::new(tar_gz);

        //     let mut archive = Archive::new(tar);

        //     for entry in archive.entries()? {
        //         // Make sure there wasn't an I/O error
        //         let mut entry = entry?;

        //         entry.unpack_in(&out_dir)?;

        //         // if tar::EntryType::Regular == entry.header().entry_type() {
        //         //     println!(">>>>>> - {}", entry.path().unwrap().display());
        //         // }
        //     }
        // }
    }
    Ok(())
}
