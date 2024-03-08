# FileCat
![Crates.io Version](https://img.shields.io/crates/v/file_cat?link=https%3A%2F%2Fcrates.io%2Fcrates%2Ffile_cat)

FileCat is a command-line tool written in Rust that allows you to concatenate the contents of files with a specified extension.
This was originally created to concatenate project directories for LLM context and training data.

## Usage

To use FileCat, run the compiled binary with the desired file extension as an argument. You can also specify the path to search for files (default is the current directory) and exclude specific directories from the search.

```shell
Usage: file_cat [OPTIONS] <FILE_EXTENSION>

Arguments:
  <FILE_EXTENSION>  Files with this extension will be collected

Options:
  -p, --path <PATH>            The path to search for files [default: .]
  -e, --exclude <EXCLUDE>      Directories to exclude from the search, comma separated [default: ]
  -r, --recursive              Search recursively
  -m, --max-depth <MAX_DEPTH>  Max Recursion Depth [default: 100]
  -s, --strip-newlines         Strip newlines from the output
  -c, --no-comments            Hide filename comments
  -d  --default-only           Only use the default handler, instead of the handler for the specified extension
  -h, --help                   Print help
  -V, --version                Print version
```
Examples:

```shell
file_cat rs -r
file_cat txt /path/to/directory
file_cat js /path/to/directory -e node_modules,dist
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
file_cat rs -r > output.txt
```

## Handlers

FileCat supports extension-specific handlers for processing files. Currently, the following handlers are available:

    csv_handler: Concatenates CSV files by aligning the headers and columns.
    default: The default handler, which concatenates files as-is with optional filename comments.

By default, FileCat will use the appropriate handler based on the file extension. If you want to force using the default handler regardless of the extension, you can use the --default-only flag.

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

## License

This project is licensed under the MIT License.
