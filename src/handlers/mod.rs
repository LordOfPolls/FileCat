use std::fs::DirEntry;

mod csv_handler;
mod default;

pub fn concat(
    extension: String,
    files: Vec<DirEntry>,
    hide_filename_comments: bool,
    strip_newlines: bool,
    default_only: bool,
) -> Result<(), std::io::Error> {
    if default_only {
        return default::concat(extension, files, hide_filename_comments, strip_newlines);
    }

    match extension.as_str() {
        "csv" => csv_handler::concat(files)?,
        _ => default::concat(extension, files, hide_filename_comments, strip_newlines)?,
    }
    Ok(())
}
