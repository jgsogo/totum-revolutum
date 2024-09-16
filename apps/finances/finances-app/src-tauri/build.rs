use std::env;

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    // FIXME: Change only if running inside Bazel
    env::set_current_dir(path.join("wf-backend")).unwrap();
    println!("The current directory is {}", env::current_dir().unwrap().display());

    tauri_build::build()
}
