use std::{
    env,
    fs::File,
    io::{self, Read},
    process,
};

fn exit(msg: &str) {
    println!("{}", msg);
    process::exit(1);
}

fn search(needle: &str, haystack: impl Iterator<Item = String>) {
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

fn search_files(needle: &str, file_paths: Vec<&String>) {
    for file_path in file_paths {
        if let Ok(mut file) = File::open(file_path) {
            let mut buf = String::new();
            let _ = file.read_to_string(&mut buf);
            search(needle, buf.lines().map(|s| s.to_owned()))
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
        _ => search_files(&args[1], args.iter().skip(2).collect::<Vec<&String>>()),
    }

    Ok(())
}
