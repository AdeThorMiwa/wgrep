use std::{
    env,
    fs::File,
    io::{self, Read},
    path::Path,
    process,
};

fn exit(msg: &str) {
    println!("{}", msg);
    process::exit(1);
}

fn search<H, I>(needle: &str, haystack: I)
where
    H: std::ops::Deref<Target = str> + std::fmt::Display,
    I: Iterator<Item = H>,
{
    for line in haystack {
        if line.contains(needle) {
            println!("{}", line)
        }
    }
}

fn search_stdin(needle: &str) {
    let lines = io::stdin().lines();
    search(needle, lines.map(|l| l.unwrap()))
}

fn search_files<P, I>(needle: &str, file_paths: I)
where
    P: AsRef<Path>,
    I: Iterator<Item = P>,
{
    let mut buf = String::new();
    for file_path in file_paths {
        if let Ok(mut file) = File::open(file_path) {
            file.read_to_string(&mut buf).unwrap();
            search(needle, buf.lines())
        } else {
            exit("wgrep: cannot open file")
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => exit("wgrep: searchterm [file ...]"),
        2 => search_stdin(&args[1]),
        _ => search_files(&args[1], args.iter().skip(2)),
    }

    Ok(())
}
