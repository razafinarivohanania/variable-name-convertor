mod args;

fn main() {
    match args::Args::new() {
        Ok(args) => println!("{}", args.to_string()),
        Err(error) => println!("Error occured : {}", error),
    }
}
