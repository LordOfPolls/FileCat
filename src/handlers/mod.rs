use std::fs::DirEntry;

mod default;

pub fn concat(
    extension: String,
    files: Vec<DirEntry>,
    hide_filename_comments: bool,
    strip_newlines: bool,
) -> Result<(), std::io::Error> {
    match extension {
        _ => default::concat(extension, files, hide_filename_comments, strip_newlines)?,
    }
    Ok(())
}
