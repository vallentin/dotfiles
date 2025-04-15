#![forbid(unsafe_code, elided_lifetimes_in_paths)]
#![allow(dead_code)]
#![cfg_attr(
    debug_assertions,
    allow(unreachable_code, unused_imports, unused_variables, unused_mut)
)]
#![warn(clippy::all)]

use std::error;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

use std::os::unix::fs::symlink;

const BIN_DIR: &str = "~/bin";

#[rustfmt::skip]
const TOOLS: &[(&str, fn() -> Result<(), Box<dyn error::Error>>)] = &[
    ("tools", run),
    ("dummy", dummy),
];

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

    let exe = std::env::current_exe().unwrap();

    let bin_dir = shellexpand::tilde(BIN_DIR);
    let bin_dir = Path::new(bin_dir.as_ref());

    fs::create_dir_all(bin_dir).unwrap();

    for name in TOOLS.iter().map(|&(name, _)| name) {
        install_tool(name, &exe, bin_dir)?;
    }

    Ok(())
}

fn install_tool(tool_name: &str, exe: &Path, bin_dir: &Path) -> Result<(), Box<dyn error::Error>> {
    let link = bin_dir.join(tool_name);

    match fs::remove_file(&link) {
        Ok(()) => {
            println!("Removed    `{}`", link.display());
        }
        Err(err) if err.kind() == io::ErrorKind::NotFound => {}
        Err(err) => return Err(err.into()),
    }

    println!("Installing `{}`", link.display());
    symlink(&exe, &link).unwrap();

    Ok(())
}

fn dummy() -> Result<(), Box<dyn error::Error>> {
    println!("DUMMY");

    Ok(())
}
