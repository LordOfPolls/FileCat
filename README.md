# FileCat
![Crates.io Version](https://img.shields.io/crates/v/file_cat?link=https%3A%2F%2Fcrates.io%2Fcrates%2Ffile_cat)

FileCat is a command-line tool written in Rust that allows you to concatenate the contents of files with a specified extension and display them in the console. It provides a convenient way to view the contents of multiple files at once, along with their file paths and names.
Features

- Specify the file extension to filter files
- Recursively search for files in subdirectories
- Exclude specific directories from the search
- Display file paths, names, and contents in the console
- Automatic detection of comment syntax based on file extension
- Cross-platform compatibility (Windows, macOS, Linux)

## Installation

To use FileCat, you need to have Rust installed on your system. If you don't have Rust installed, you can download and install it from the official Rust website: https://www.rust-lang.org

Once Rust is installed, you can clone this repository and build the project using Cargo, the Rust package manager:

### From Crates.io
```shell
cargo install file_cat
```

### From Source
```shell
git clone https://github.com/your-username/file_cat.git
cd file_cat
cargo install --path .    
```

## Usage

To use FileCat, run the compiled binary with the desired file extension as an argument. You can also specify the path to search for files (default is the current directory) and exclude specific directories from the search.

```shell
file_cat <extension> [path] [excluded_dirs...]

    <extension>: The file extension to filter files (e.g., "txt", "rs", "js"). Use "*" to include all files.
    [path]: Optional. The path to search for files (default is the current directory).
    [excluded_dirs...]: Optional. The directories to exclude from the search (space-separated).
```
Examples:

```shell
file_cat rs
file_cat txt /path/to/directory
file_cat js /path/to/directory node_modules dist
```


## Output

FileCat will display the file paths, names, and contents in the console. It automatically detects the appropriate comment syntax based on the file extension and includes it in the output.

Example output:
```
// File: src/main.rs
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn collect_files_with_extension(
extension: &str,
recursive: bool,
path: &Path,
excluded_dirs: &[String],
) -> io::Result<Vec<fs::DirEntry>> {
// ...
}

// ...
```

To store the output as a file, normal syntax applies for redirecting the output to a file:

```shell
file_cat rs > output.txt
```

## License

This project is licensed under the MIT License.