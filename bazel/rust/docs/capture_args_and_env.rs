// Copyright 2023 The Bazel Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const ENVVAR_PREFIX: &str = "env: ";
const PARAMS_FILE_MARKER: &str = "@";
const OUTPUT_FILE_ENVVAR: &str = "OUTPUT_FILE";

fn collect_env_vars(envvars: impl Iterator<Item = (String, String)>, mut writer: impl std::io::Write) {
    for (key, value) in envvars {
        writeln!(&mut writer, "{}{}={}", ENVVAR_PREFIX, key, value).expect("Unable to write");
    }
}

fn collect_env_args(envargs: impl Iterator<Item = String>, mut writer: impl std::io::Write) {
    // Skip first argument, it's our executable path
    let mut envargs: Vec<String> = envargs.skip(1).collect();

    // If last argument starts with a '@' it's a params file (see https://bazel.build/rules/lib/builtins/Args.html),
    // we need to follow the pointer and collect the params stored in that file as well
    let mut last_arg = envargs.last().unwrap().clone();
    if last_arg.starts_with(PARAMS_FILE_MARKER) {
        last_arg.remove(0);
        envargs.pop();

        let file = File::open(last_arg).expect("Cannot open parameters file");
        let buf = BufReader::new(file);
        let mut lines: Vec<_> = buf.lines().map(|l| l.expect("Could not parse line")).collect();
        envargs.append(&mut lines);
    }
    writeln!(&mut writer, "{}", envargs.join("\n")).expect("Unable to write");
}

// Simple utility to capture command line arguments and write them to a file.
pub fn main() {
    let file_name =
        env::var(OUTPUT_FILE_ENVVAR).expect(&format!("{} environment variable is not set", OUTPUT_FILE_ENVVAR));
    let mut file = File::create(file_name).expect(&format!("Can't open {}", OUTPUT_FILE_ENVVAR));

    // Write environment variables to the file as well, use keyword "env: " to idenfity them
    collect_env_vars(env::vars(), &mut file);

    // Write command line args, skipping the first (our executable path), to
    // `OUTPUT_FILE` separated by newlines.
    collect_env_args(env::args(), &mut file);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_env_vars() {
        let mut result = Vec::new();
        let envvars = vec![
            ("k1".to_string(), "v1".to_string()),
            ("k2".to_string(), "v2".to_string()),
        ];
        collect_env_vars(envvars.into_iter(), &mut result);

        assert_eq!(String::from_utf8(result).unwrap(), "env: k1=v1\nenv: k2=v2\n");
    }
}
