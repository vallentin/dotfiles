#![forbid(unsafe_code, elided_lifetimes_in_paths)]
#![allow(dead_code)]
#![cfg_attr(
    debug_assertions,
    allow(unreachable_code, unused_imports, unused_variables, unused_mut)
)]
#![warn(clippy::all)]

mod pretty;
mod utils;

use std::error;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

use std::os::unix::fs::symlink;

const BIN_DIR: &str = "~/.val/bin";

const TOOLS: &[(&str, fn() -> Result<(), Box<dyn error::Error>>)] = &[
    ("tools", run),
    ("reinstall-tools", install),
    ("pretty-json", crate::pretty::json),
    ("dummy", dummy),
];

#[cfg(debug_assertions)]
const PROFILE: &str = "debug";
#[cfg(not(debug_assertions))]
const PROFILE: &str = "release";

fn main() {
    let program = PathBuf::from(std::env::args_os().next().unwrap());
    let program = program.file_stem().unwrap().to_str().unwrap();

    exit({
        let code = match try_main(program) {
            Ok(()) => 0,
            Err(err) => {
                eprintln!("error: {err}");
                1
            }
        };
        let _ = io::stdout().flush();
        let _ = io::stderr().flush();
        code
    });
}

fn try_main(program: &str) -> Result<(), Box<dyn error::Error>> {
    let run = TOOLS
        .iter()
        .find_map(|&(name, run)| (name == program).then_some(run))
        .ok_or_else(|| format!("unknown tool `{program}`"))?;

    run()?;

    Ok(())
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let cmd = std::env::args().nth(1);
    let cmd = cmd.as_deref().unwrap_or("list");

    match cmd {
        "list" | "tools" => list_tools(),
        "install" => install(),
        _ => Err(format!("unknown command `{cmd}`").into()),
    }
}

fn list_tools() -> Result<(), Box<dyn error::Error>> {
    let mut names = TOOLS.iter().map(|&(name, _)| name).collect::<Vec<_>>();
    names.sort();

    for name in names {
        println!("{name}");
    }

    Ok(())
}

fn install() -> Result<(), Box<dyn error::Error>> {
    println!("Installing tools...");

    let exe = exe_path(PROFILE);

    let bin_dir = shellexpand::tilde(BIN_DIR);
    let bin_dir = Path::new(bin_dir.as_ref());

    fs::create_dir_all(bin_dir).unwrap();

    for name in TOOLS.iter().map(|&(name, _)| name) {
        install_tool(name, &exe, bin_dir)?;
        install_tool_debug(name, bin_dir)?;
    }

    Ok(())
}

fn install_tool(tool_name: &str, exe: &Path, bin_dir: &Path) -> Result<(), Box<dyn error::Error>> {
    let link = bin_dir.join(tool_name);

    if utils::try_remove_file(&link)? {
        println!("Removed    `{}`", link.display());
    }

    println!("Installing `{}`", link.display());
    symlink(&exe, &link).unwrap();

    Ok(())
}

fn install_tool_debug(tool_name: &str, bin_dir: &Path) -> Result<(), Box<dyn error::Error>> {
    let manifest_path = env!("CARGO_MANIFEST_PATH");
    let manifest_path = shlex::try_quote(manifest_path).unwrap();

    let sh = bin_dir.join(format!("_{tool_name}"));

    if utils::try_remove_file(&sh)? {
        println!("Removed    `{}`", sh.display());
    }

    let code = [
        "#!/usr/bin/env bash",
        "set -e",
        #[cfg(debug_assertions)]
        &format!("cargo build --manifest-path {manifest_path}"),
        #[cfg(not(debug_assertions))]
        &format!("cargo build --release --manifest-path {manifest_path}"),
        &format!("{tool_name} \"$@\""),
    ]
    .join("\n");

    println!("Installing `{}`", sh.display());
    fs::write(&sh, code).unwrap();

    utils::mark_executable(&sh)?;

    Ok(())
}

fn dummy() -> Result<(), Box<dyn error::Error>> {
    println!("DUMMY");

    Ok(())
}

fn exe_path(profile: &str) -> PathBuf {
    // Not using `std::env::current_exe()` as executing `reinstall-tools`
    // results in the wrong executable path
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(profile)
        .join(env!("CARGO_BIN_NAME"))
}
