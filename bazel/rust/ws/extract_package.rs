use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;

use toml_edit::{DocumentMut, InlineTable, Table};

use anyhow::{bail, Result};
use std::io::Write;

/// Utility to generate a standalone Cargo.toml for a package that belongs to a workspace
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the workspace Cargo.toml file
    #[arg(long)]
    workspace: Utf8PathBuf,

    /// Path to the package Cargo.toml file
    #[arg(long)]
    package: Utf8PathBuf,

    /// Path to the generated Cargo.toml file
    #[arg(long)]
    output: Utf8PathBuf,
}

// #[derive(Deserialize, Serialize)]
// struct WorkspaceConfig {
//     workspace: Workspace,

//     #[serde(rename = "workspace.dependencies")]
//     dependencies: HashMap<String, String>,
// }

// #[derive(Deserialize, Serialize)]
// struct Workspace {
//     members: Vec<String>,
// }

struct PackageConfig {
    pub package: Table,
    pub lib: Table,
    pub build_deps: Table,
    pub dependencies: Table,
}

struct WorkspaceConfig {
    // pub workspace: Table,
    pub dependencies: Table,
}

fn parse_package(config_file: &Utf8Path) -> Result<PackageConfig> {
    let content: String = std::fs::read_to_string(config_file)?;
    let document: DocumentMut = content.parse()?;
    // println!("{}", document.to_string());
    let config = PackageConfig {
        package: document["package"].clone().into_table().unwrap(),
        lib: document["lib"].clone().into_table().unwrap(),
        build_deps: document["build-dependencies"].clone().into_table().unwrap(),
        dependencies: document["dependencies"].clone().into_table().unwrap(),
    };
    Ok(config)
}

fn parse_workspace(config_file: &Utf8Path) -> Result<WorkspaceConfig> {
    let content: String = std::fs::read_to_string(config_file)?;
    let document: DocumentMut = content.parse()?;
    // println!("{}", document.to_string());
    let config = WorkspaceConfig {
        // workspace: document["workspace"].clone().into_table().unwrap(),
        dependencies: document["workspace"]["dependencies"].clone().into_table().unwrap(),
    };
    Ok(config)
}

// Simple utility to extract a package from a workspace
pub fn main() -> Result<()> {
    println!("Extract package from workspace");

    let args = Args::parse();
    let package_config = parse_package(&args.package)?;
    let ws_config = parse_workspace(&args.workspace)?;

    // TODO: Check the package belongs to the workspace
    // TODO: The Cargo.toml file is "listed" in the workspace members

    let mut output = std::fs::File::create(&args.output).expect("Unable to create file");
    write!(output, "[package]\n")?;
    write!(output, "{}\n", package_config.package.to_string())?;
    write!(output, "[lib]\n")?;
    write!(output, "{}\n", package_config.lib.to_string())?;
    write!(output, "[build-dependencies]\n")?;
    write!(output, "{}\n", package_config.build_deps.to_string())?;
    write!(output, "[dependencies]\n")?;

    for (name, pkg_item) in package_config.dependencies.iter() {
        match ws_config.dependencies.get(&name) {
            Some(ws_item) => {
                let mut item = InlineTable::new();

                // take the version from the worspace
                let version: String = if let Some(as_table) = ws_item.as_inline_table() {
                    as_table.get("version").unwrap().to_string()
                } else {
                    ws_item.to_string()
                };
                let version = version.trim().trim_matches('"');
                item.insert("version", version.into());

                // take everything else from the package
                for (k, v) in pkg_item.as_inline_table().unwrap() {
                    if k != "workspace" {
                        item.insert(k, v.into());
                    }
                }

                // print the dependency to the file
                write!(output, "{} = {}\n", name, item)?;
            }
            None => bail!(format!("Dep {} not found in workspace", name)),
        }
    }
    drop(output);

    Ok(())
}
