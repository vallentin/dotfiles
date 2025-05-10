pub mod ansi;
pub mod color;

use std::fs;
use std::io;
use std::path::Path;

use std::os::unix::fs::PermissionsExt;

const S_IXUSR: u32 = 0o0100; // Execute by owner
const S_IXGRP: u32 = 0o0010; // Execute by group
const S_IXOTH: u32 = 0o0001; // Execute by others

const EXECUTABLE: u32 = S_IXUSR | S_IXGRP | S_IXOTH;

/// Returns `Ok(true)` if `path` was removed,
/// `Ok(false)` if `path` was not found.
#[inline]
pub fn try_remove_file(path: impl AsRef<Path>) -> io::Result<bool> {
    match fs::remove_file(path) {
        Ok(()) => Ok(true),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(err) => Err(err),
    }
}

pub fn mark_executable(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();
    let metadata = path.metadata()?;

    let mut perm = metadata.permissions();
    let mode = perm.mode() | S_IXUSR | S_IXGRP | S_IXOTH;
    perm.set_mode(mode);

    fs::set_permissions(path, perm)?;

    Ok(())
}

pub fn is_executable(path: impl AsRef<Path>) -> io::Result<bool> {
    let metadata = path.as_ref().metadata()?;
    let mode = metadata.permissions().mode();
    Ok((mode & EXECUTABLE) != 0)
}
