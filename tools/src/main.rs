#![forbid(unsafe_code, elided_lifetimes_in_paths)]
#![allow(dead_code)]
#![cfg_attr(
    debug_assertions,
    allow(unreachable_code, unused_imports, unused_variables, unused_mut)
)]
#![warn(clippy::all)]

mod pretty;
mod tool;
mod utils;
mod walk;

use std::env;
use std::error;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::exit;

use crate::tool::{Tool, ToolFn};

const TOOLS: &[Tool] = &[
    t("tools", run),
    t("list-tools", list_tools),
    t("reinstall-tools", install),
    t("uninstall-tools", uninstall),
    t("pretty-json", crate::pretty::json),
    t("dir", crate::walk::run),
    t("tree", crate::walk::run),
    t("walk", crate::walk::run),
    t("dummy", dummy),
];

#[inline]
const fn t(name: &'static str, func: ToolFn) -> Tool {
    Tool::new(name, func)
}

#[cfg(debug_assertions)]
const PROFILE: &str = "debug";
#[cfg(not(debug_assertions))]
const PROFILE: &str = "release";

fn main() {
    let program = PathBuf::from(env::args_os().next().unwrap());
    let program = program.file_stem().unwrap().to_str().unwrap();

    let force_tool = env::var("FORCE_TOOL").ok();
    let program = match &force_tool {
        Some(tool_name) => tool_name.as_str(),
        None => program,
    };

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
        .find_map(|tool| (tool.name() == program).then(|| tool.func()))
        .ok_or_else(|| format!("unknown tool `{program}`"))?;

    run()?;

    Ok(())
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let cmd = env::args().nth(1);
    let cmd = cmd.as_deref().unwrap_or("list");

    match cmd {
        "list" | "tools" => list_tools(),
        "install" | "reinstall-tools" => install(),
        "uninstall" => uninstall(),
        _ => Err(format!("unknown command `{cmd}`").into()),
    }
}

fn list_tools() -> Result<(), Box<dyn error::Error>> {
    let mut names = TOOLS.iter().map(Tool::name).collect::<Vec<_>>();
    names.sort();

    for name in names {
        println!("{name}");
    }

    Ok(())
}

fn install() -> Result<(), Box<dyn error::Error>> {
    uninstall()?;

    println!("Installing tools...");

    for tool in TOOLS {
        tool.install()?;
        tool.install_debug()?;
    }

    Ok(())
}

fn uninstall() -> Result<(), Box<dyn error::Error>> {
    println!("Uninstalling tools...");

    for tool in TOOLS {
        tool.uninstall().unwrap();
        tool.uninstall_debug().unwrap();
    }

    Ok(())
}

fn dummy() -> Result<(), Box<dyn error::Error>> {
    println!("DUMMY");

    Ok(())
}
