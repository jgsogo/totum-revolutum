use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

// Simple utility to run commands from files.  Each command/command line
// argument in the file is on a separate line.
pub fn main() {
    // Skip the first arg (our executable path).
    let scripts = env::args().skip(1);

    for script in scripts {
        let script_file = BufReader::new(File::open(&script).expect("unable to open file"));
        let lines = script_file
            .lines()
            .map(|l| l.map_err(|e| format!("{script}: error reading line: {e}")).unwrap());

        // Collect environment variables
        let (env, args_lines): (Vec<_>, Vec<_>) = lines.partition(|l| l.starts_with("env: "));
        let env_vars: HashMap<String, String> = env
            .into_iter()
            .map(|e| {
                let (k, v) = e
                    .strip_prefix("env: ")
                    .expect("doesn't start with prefix")
                    .split_once('=')
                    .unwrap();
                (k.to_string(), v.to_string())
            })
            .collect();

        // First line of the file is the command to run.
        let mut args_lines_it = args_lines.iter();
        let command = args_lines_it
            .next()
            .ok_or_else(|| format!("{script}: no command in file"))
            .unwrap();

        // Subsequent lines are arguments.
        let args = args_lines_it.collect::<Vec<_>>();

        // Run the command and wait for it to finish.
        let mut child = Command::new(&command)
            .args(args)
            .envs(&env_vars)
            .spawn()
            .map_err(|e| format!("{script}: failed to spawn child process {command}: {e}"))
            .unwrap();
        let status = child.wait().expect("");

        if !status.success() {
            panic!("{}: error running script: {}", script, status);
        }
    }
}
