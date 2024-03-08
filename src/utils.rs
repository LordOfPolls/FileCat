use std::path::Path;
use std::{env, io};

pub fn get_comment_syntax(extension: &str) -> (&str, &str) {
    match extension {
        "py" | "sh" | "rb" | "pl" | "r" | "jl" | "gnuplot" | "toml" | "ini" | "cfg" | "conf"
        | "properties" | "yaml" | "yml" | "hcl" | "tf" | "nix" => ("#", ""),
        "c" | "cpp" | "h" | "hpp" | "java" | "js" | "ts" | "go" | "php" | "swift" | "kt" | "rs"
        | "fs" | "fsx" | "fsi" | "cs" | "dart" | "scala" | "groovy" | "v" | "hs" | "elm"
        | "erl" | "hrl" | "asciidoc" | "adoc" | "json" | "jsonc" | "cson" | "proto" | "hocon"
        | "json5" | "hjson" => ("//", ""),
        "html" | "fsproj" | "xml" | "svg" | "xhtml" | "xaml" | "aspx" | "jsp" | "jspx" | "gsp"
        | "xsd" | "dtd" | "xsl" | "xslt" | "mathml" => ("<!--", "-->"),
        "css" | "scss" | "sass" | "less" | "stylus" => ("/*", "*/"),
        "lua" | "sql" | "ada" | "applescript" | "hive" | "pig" | "vb" | "dhall" => ("--", ""),
        "coffee" | "litcoffee" => ("###", ""),
        "nim" => ("##", ""),
        "edn" | "clj" | "cljs" | "cljc" | "s" | "S" | "inc" | "ahk" | "scm" | "sch" | "rkt"
        | "sld" => (";", ""),
        "ml" | "mli" | "fsscript" => ("(*", "*)"),
        "tex" | "sty" => ("%", ""),
        "m4" => ("dnl", ""),
        "csv" | "tsv" => ("", ""),
        _ => ("//", ""), // default to C-style comments
    }
}

pub fn get_path_separator() -> &'static str {
    if cfg!(windows) {
        "\\"
    } else {
        "/"
    }
}

pub fn get_relative_path(path: &Path) -> io::Result<String> {
    let current_dir = env::current_dir()?;
    let relative_path = path.strip_prefix(current_dir).unwrap_or(path);
    Ok(relative_path.to_string_lossy().into_owned())
}
