use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "Sridhar Ratnakumar", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    #[arg(short = 'n', long="no-open", default_value_t = false)]
    pub no_open: bool,

    /// an optional name to green
    #[arg()]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }
    println!("Hello {}!", args.name.unwrap_or("world".to_string()));

    let path = "http://rust-lang.org";
    if !args.no_open{
        match open::that(path) {
            Ok(()) => println!("Opened '{}' successfully.", path),
            Err(err) => eprintln!("An error occurred when opening '{}': {}", path, err),
        }
    }
}
