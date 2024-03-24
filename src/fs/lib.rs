// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

//! Wrapper around `std::fs`, `tokio::fs`, `fs_extra` and `dirs`.

use mintaka_error::Result;
use std::env;
use std::fs::{Metadata, Permissions};
use std::path::{Path, PathBuf};

pub mod paths;

/// Return the canonical absolute path.
pub fn normalize_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    Ok(dunce::canonicalize(path)?)
}

/// Returns the path to the executable path.
pub fn get_executable_path() -> Result<PathBuf> {
    Ok(normalize_path(env::current_exe()?)?)
}

pub type FileCopyOptions = fs_extra::file::CopyOptions;
pub type DirectoryCopyOptions = fs_extra::dir::CopyOptions;

/// Changes the permissions found on a file or a directory.
pub fn set_permissions<P: AsRef<Path>>(path: P, perm: Permissions) -> Result<()> {
    Ok(std::fs::set_permissions(path, perm)?)
}

#[cfg(feature = "async")]
pub async fn set_permissions_async<P: AsRef<Path>>(path: P, perm: Permissions) -> Result<()> {
    Ok(tokio::fs::set_permissions(path, perm).await?)
}

/// Queries the file system metadata for a path.
pub fn get_metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
    Ok(std::fs::metadata(path)?)
}

#[cfg(feature = "async")]
pub async fn get_metadata_async<P: AsRef<Path>>(path: P) -> Result<Metadata> {
    Ok(tokio::fs::metadata(path).await?)
}

/// Queries the file system metadata for a path.
pub fn get_symlink_metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
    Ok(std::fs::symlink_metadata(path)?)
}

#[cfg(feature = "async")]
pub async fn get_symlink_metadata_async<P: AsRef<Path>>(path: P) -> Result<Metadata> {
    Ok(tokio::fs::symlink_metadata(path).await?)
}

/// Copy file to location.
pub fn copy_file<P: AsRef<Path>>(orig: P, dest: P) -> Result<u64> {
    Ok(std::fs::copy(orig, dest)?)
}

#[cfg(feature = "async")]
pub async fn copy_file_async<P: AsRef<Path>>(orig: P, dest: P) -> Result<u64> {
    Ok(tokio::fs::copy(orig, dest).await?)
}

/// Copy directory to location.
pub fn copy_directory<P: AsRef<Path>>(
    orig: P,
    dest: P,
    options: &DirectoryCopyOptions,
) -> fs_extra::error::Result<u64> {
    Ok(fs_extra::dir::copy(orig, dest, options)?)
}

/// Create directory recursively.
pub fn create_directory<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(std::fs::create_dir_all(path)?)
}

#[cfg(feature = "async")]
pub async fn create_directory_async<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(tokio::fs::create_dir_all(path).await?)
}

/// Creates a new hard link on the filesystem.
pub fn create_hard_link<P: AsRef<Path>>(orig: P, dest: P) -> Result<()> {
    Ok(std::fs::hard_link(orig, dest)?)
}

#[cfg(feature = "async")]
pub async fn create_hard_link_async<P: AsRef<Path>>(orig: P, dest: P) -> Result<()> {
    Ok(tokio::fs::hard_link(orig, dest).await?)
}

/// Creates a new file symbolic link on the filesystem.
pub fn create_symbolic_file<P: AsRef<Path>>(orig: P, dest: P) -> Result<()> {
    #[cfg(target_family = "windows")]
    return Ok(std::os::windows::fs::symlink_file(orig, dest)?);

    #[cfg(target_family = "unix")]
    return Ok(std::os::unix::fs::symlink(orig, dest)?);
}

#[cfg(feature = "async")]
pub async fn create_symbolic_file_async<P: AsRef<Path>>(orig: P, dest: P) -> Result<()> {
    #[cfg(target_family = "windows")]
    return Ok(tokio::fs::symlink_file(orig, dest).await?);

    #[cfg(target_family = "unix")]
    return Ok(tokio::fs::symlink(orig, dest).await?);
}

/// Creates a new directory symlink on the filesystem.
pub fn create_symbolic_directory<P: AsRef<Path>>(orig: P, link: P) -> Result<()> {
    #[cfg(target_family = "windows")]
    return Ok(std::os::windows::fs::symlink_dir(orig, link)?);

    #[cfg(target_family = "unix")]
    return Ok(std::os::unix::fs::symlink(orig, link)?);
}

#[cfg(feature = "async")]
pub async fn create_symbolic_directory_async<P: AsRef<Path>>(orig: P, link: P) -> Result<()> {
    #[cfg(target_family = "windows")]
    return Ok(tokio::fs::symlink_dir(orig, link).await?);

    #[cfg(target_family = "unix")]
    return Ok(tokio::fs::symlink(orig, link).await?);
}

/// Read file as binary buffer.
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    Ok(std::fs::read(path)?)
}

#[cfg(feature = "async")]
pub async fn read_file_async<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    Ok(tokio::fs::read(path).await?)
}

/// Read directory contents.
pub fn read_directory<P: AsRef<Path>>(path: P) -> Result<std::fs::ReadDir> {
    Ok(std::fs::read_dir(path)?)
}

#[cfg(feature = "async")]
pub async fn read_directory_async<P: AsRef<Path>>(path: P) -> Result<tokio::fs::ReadDir> {
    Ok(tokio::fs::read_dir(path).await?)
}

/// Read file content as string.
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

#[cfg(feature = "async")]
pub async fn read_to_string_async<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(tokio::fs::read_to_string(path).await?)
}

/// Delete file at location.
pub fn delete_file<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(std::fs::remove_file(path)?)
}

#[cfg(feature = "async")]
pub async fn delete_file_async<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(tokio::fs::remove_file(path).await?)
}

/// Delete directory and its contents recursively.
pub fn delete_directory<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(std::fs::remove_dir_all(path)?)
}

#[cfg(feature = "async")]
pub async fn delete_directory_async<P: AsRef<Path>>(path: P) -> Result<()> {
    Ok(tokio::fs::remove_dir_all(path).await?)
}

/// Return information about directory:.
///
/// * Directory size.
/// * List all directory and subdirectories.
/// * List all files path.
pub fn get_directory_content<P: AsRef<Path>>(
    path: P,
) -> fs_extra::error::Result<fs_extra::dir::DirContent> {
    Ok(fs_extra::dir::get_dir_content(path)?)
}

/// Rename file or directory.
pub fn rename<P: AsRef<Path>>(old: P, new: P) -> Result<()> {
    Ok(std::fs::rename(old, new)?)
}

/// Move file to location.
pub fn move_file<P: AsRef<Path>>(
    orig: P,
    dest: P,
    options: &FileCopyOptions,
) -> fs_extra::error::Result<u64> {
    Ok(fs_extra::file::move_file(orig, dest, options)?)
}

/// Move directory to location.
pub fn move_directory<P: AsRef<Path>>(
    orig: P,
    dest: P,
    options: &DirectoryCopyOptions,
) -> fs_extra::error::Result<u64> {
    Ok(fs_extra::dir::move_dir(orig, dest, options)?)
}

#[cfg(feature = "async")]
pub async fn rename_async<P: AsRef<Path>>(old: P, new: P) -> Result<()> {
    Ok(tokio::fs::rename(old, new).await?)
}

/// Write contents to file.
pub fn write_file<P, C>(path: P, content: C) -> Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    Ok(std::fs::write(path, content)?)
}

#[cfg(feature = "async")]
pub async fn write_file_async<P, C>(path: P, content: C) -> Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    Ok(tokio::fs::write(path, content).await?)
}
