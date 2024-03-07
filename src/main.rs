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
    let mut files = Vec::new();

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && (path.extension().map_or(false, |ext| ext.to_str().unwrap() == extension || extension == "*")) {
            files.push(entry);
        } else if recursive && path.is_dir() && !excluded_dirs.contains(&path.file_name().unwrap().to_string_lossy().into_owned()) {
            let mut sub_files = collect_files_with_extension(extension, recursive, &path, excluded_dirs)?;
            files.append(&mut sub_files);
        }
    }

    Ok(files)
}

fn get_comment_syntax(extension: &str) -> (&str, &str) {
    match extension {
        "py" | "sh" | "rb" | "pl" | "r" | "jl" => ("#", ""),
        "c" | "cpp" | "h" | "hpp" | "java" | "js" | "ts" | "go" | "php" | "swift" | "kt" | "rs" | "fs" | "fsx" | "fsi" | "cs" | "dart" | "scala" | "groovy" | "v" | "hs" | "elm" | "erl" | "hrl" => ("//", ""),
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
        _ => ("//", "")
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

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        writeln!(io::stderr(), "Please provide the file extension as an argument. Expected usage: file_cat <extension> [path] [excluded_dirs...]")?;
        return Ok(());
    }

    let extension = &args[1];
    let path = &args.get(2).map_or(".", |p| p.as_str());
    let excluded_dirs: Vec<String> = args.iter().skip(3).cloned().collect();
    let system_path_separator = get_path_separator();

    env::set_current_dir(path)?;

    let files = collect_files_with_extension(extension, true, &env::current_dir()?, &excluded_dirs)?;

    if files.is_empty() {
        writeln!(io::stderr(), "No files found with extension: {}", extension)?;
        return Ok(());
    }

    for file in files {
        let file_name = file.file_name().into_string().unwrap();
        let relative_path = get_relative_path(&file.path())?;
        let (comment_prefix, comment_suffix) = get_comment_syntax(extension);

        writeln!(io::stdout(), "{} File: {}{}{} {}", comment_prefix, relative_path, system_path_separator, file_name, comment_suffix)?;
        let input = File::open(file.path())?;
        let reader = BufReader::new(input);

        for line in reader.lines() {
            let line = line?;
            writeln!(io::stdout(), "{}", line)?;
        }

        writeln!(io::stdout())?;
    }

    Ok(())
}