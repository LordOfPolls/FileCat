use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use clap::Parser;

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
        if path.is_file() && (path.extension().map_or(false, |ext| extension == "*" || ext.to_str().unwrap().to_lowercase() == extension)) {
            files.push(entry);
        } else if recursive && path.is_dir() && !excluded_dirs.contains(&path.file_name().unwrap().to_string_lossy().into_owned()) {
            let mut sub_files = collect_files_with_extension(extension, recursive, &path, excluded_dirs, depth + 1, max_depth)?;
            files.append(&mut sub_files);
        }
    }

    Ok(files)
}

fn get_comment_syntax(extension: &str) -> (&str, &str) {
    match extension {
        "py" | "sh" | "rb" | "pl" | "r" | "jl" => ("#", ""),
        "c" | "cpp" | "h" | "hpp" | "java" | "js" | "ts" | "go" | "php" | "swift" | "kt" | "rs" | "fs" | "fsx" | "fsi" | "cs" | "dart" | "scala" | "groovy" | "v" | "hs" | "elm" | "erl" | "hrl" => ("//", ""), // could probably just leave this for the default case, but it's nice to be explicit
        "html" | "fsproj" | "xml" | "svg" | "xhtml" | "xaml" | "aspx" | "jsp" | "jspx" | "gsp" => ("<!--", "-->"),
        "css" | "scss" | "sass" | "less" | "stylus" => ("/*", "*/"),
        "lua" | "sql" | "ada" | "applescript" | "hive" | "pig" | "vb" => ("--", ""),
        "coffee" | "litcoffee" => ("###", ""),
        "nim" => ("##", ""),
        "edn" | "clj" | "cljs" | "cljc" => (";", ""),
        "ml" | "mli" | "fsscript" => ("(*", "*)"),
        "s" | "S" | "inc" => (";", ""),
        "ahk" => (";", ""),
        "tex" | "sty" => ("%", ""),
        "asciidoc" | "adoc" => ("//", ""),
        "gnuplot" => ("#", ""),
        "scm" | "sch" | "rkt" | "sld" => (";", ""),
        "m4" => ("dnl", ""),
        _ => ("//", "")  // default to C-style comments
    }
}

fn get_path_separator() -> &'static str {
    if cfg!(windows) {
        "\\"
    } else {
        "/"
    }
}

fn get_relative_path(path: &Path) -> io::Result<String> {
    let current_dir = env::current_dir()?;
    let relative_path = path.strip_prefix(current_dir).unwrap_or(path);
    Ok(relative_path.to_string_lossy().into_owned())
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
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let extension = args.file_extension.to_lowercase();
    let path = args.path;
    let excluded_dirs = args.exclude;
    let recursive = args.recursive;
    let max_depth = args.max_depth;
    let strip_newlines = args.strip_newlines;


    let excluded_dirs: Vec<String> = excluded_dirs.iter().map(|dir| dir.trim().to_string()).collect();

    let system_path_separator = get_path_separator();

    if !Path::new(&path).exists() {
        writeln!(io::stderr(), "Path does not exist: {}", path)?;
        return Ok(());
    }

    env::set_current_dir(path)?;

    let files = collect_files_with_extension(&extension, recursive, &env::current_dir()?, &excluded_dirs, 0, max_depth)?;

    if files.is_empty() {
        writeln!(io::stderr(), "No files found with extension: {}", extension)?;
        return Ok(());
    }

    for file in files {
        let file_name = file.file_name().into_string().unwrap();
        let relative_path = get_relative_path(&file.path())?;
        let (comment_prefix, comment_suffix) = get_comment_syntax(&extension);

        writeln!(io::stdout(), "")?;
        writeln!(io::stdout(), "{} File: {}{}{} {}", comment_prefix, relative_path, system_path_separator, file_name, comment_suffix)?;
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