use std::fs::DirEntry;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use crate::utils;

pub fn concat(
    extension: String,
    files: Vec<DirEntry>,
    hide_filename_comments: bool,
    strip_newlines: bool,
) -> Result<(), io::Error> {
    for file in files {
        let file_name = file.file_name().into_string().unwrap();
        let relative_path = utils::get_relative_path(&file.path())?;

        if !hide_filename_comments {
            let (comment_prefix, comment_suffix) = utils::get_comment_syntax(&extension);

            writeln!(io::stdout(), "")?;
            writeln!(
                io::stdout(),
                "{} File: {}{}{} {}",
                comment_prefix,
                relative_path,
                utils::get_path_separator(),
                file_name,
                comment_suffix
            )?;
        }

        let input = File::open(file.path())?;
        let reader = BufReader::new(input);

        for line in reader.lines() {
            let line = line?;
            if strip_newlines {
                write!(io::stdout(), "{}", line)?;
            } else {
                writeln!(io::stdout(), "{}", line)?;
            }
        }

        writeln!(io::stdout())?;
    }
    Ok(())
}
