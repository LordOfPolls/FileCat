use std::env;
use std::fs::{self};
use std::io::{self, Write};
use std::path::Path;

use clap::Parser;

mod handlers;
mod utils;

fn collect_files_with_extension(
    extension: &str,
    recursive: bool,
    path: &Path,
    excluded_dirs: &[String],
    depth: u32,
    max_depth: u32,
) -> io::Result<Vec<fs::DirEntry>> {
    let mut files = Vec::new();

    if depth > max_depth {
        return Ok(files);
    }

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
            && (path.extension().map_or(false, |ext| {
                extension == "*" || ext.to_str().unwrap().to_lowercase() == extension
            }))
        {
            files.push(entry);
        } else if recursive
            && path.is_dir()
            && !excluded_dirs.contains(&path.file_name().unwrap().to_string_lossy().into_owned())
        {
            let mut sub_files = collect_files_with_extension(
                extension,
                recursive,
                &path,
                excluded_dirs,
                depth + 1,
                max_depth,
            )?;
            files.append(&mut sub_files);
        }
    }

    Ok(files)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Files with this extension will be collected
    #[arg(index = 1)]
    file_extension: String,

    /// The path to search for files
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Directories to exclude from the search, comma separated
    #[arg(short, long, default_value = "", value_delimiter = ',')]
    exclude: Vec<String>,

    /// Search recursively
    #[arg(short, long, default_value = "false")]
    recursive: bool,

    /// Max Recursion Depth
    #[arg(short, long, default_value = "100")]
    max_depth: u32,

    /// Strip newlines from the output
    #[arg(short, long, default_value = "false")]
    strip_newlines: bool,

    /// Hide filename comments
    #[arg(short = 'c', long = "no-comments", default_value = "false")]
    hide_filename_comments: bool,

    /// Only use the default handler, instead of the handler for the specified extension
    #[arg(short, long, default_value = "false")]
    default_only: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let extension = args.file_extension.to_lowercase();
    let path = args.path;
    let excluded_dirs = args.exclude;
    let recursive = args.recursive;
    let max_depth = args.max_depth;
    let strip_newlines = args.strip_newlines;
    let hide_filename_comments = args.hide_filename_comments;

    let excluded_dirs: Vec<String> = excluded_dirs
        .iter()
        .map(|dir| dir.trim().to_string())
        .collect();

    if !Path::new(&path).exists() {
        writeln!(io::stderr(), "Path does not exist: {}", path)?;
        return Ok(());
    }

    env::set_current_dir(path)?;

    let files = collect_files_with_extension(
        &extension,
        recursive,
        &env::current_dir()?,
        &excluded_dirs,
        0,
        max_depth,
    )?;

    if files.is_empty() {
        writeln!(io::stderr(), "No files found with extension: {}", extension)?;
        return Ok(());
    }

    handlers::concat(
        extension,
        files,
        hide_filename_comments,
        strip_newlines,
        args.default_only,
    )?;

    Ok(())
}
