use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    // handling arguments manually, without clap crate
    //let pattern = std::env::args().nth(1).expect("Please provide pattern");
    //let path = std::env::args().nth(2).expect("Please provide path");
    //let args = Cli {
    //    pattern,
    //    path: std::path::PathBuf::from(path),
    //};

    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content },
        Err(error) => { panic!("Error occured while trying to open file: {}", error); }
    };

    for (i, line) in content.lines().enumerate() {
        if line.contains(&args.pattern) {
            println!("{}: {}", i + 1, line);
        }
    }
}
