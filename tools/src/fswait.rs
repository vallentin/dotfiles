use std::env;
use std::path::Path;
use std::sync::mpsc;

use notify::Error as NotifyError;
use notify::{Event, RecursiveMode, Watcher};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let paths = env::args().skip(1).collect::<Vec<_>>();
    if paths.is_empty() {
        return Err("no path(s) given".into());
    }

    let (tx, rx) = mpsc::channel::<Result<Event, NotifyError>>();

    let mut watcher = notify::recommended_watcher(tx)?;
    for path in paths {
        watcher.watch(Path::new(&path), RecursiveMode::Recursive)?;
    }

    // Wait until an event has been detected
    _ = rx.recv()??;

    Ok(())
}
