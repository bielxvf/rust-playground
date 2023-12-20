use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let ref subcommand = args.remove(1);

    match subcommand.as_str() {
        "help" => {
            if args.len() > 1 {
                println!("Too many args");
                std::process::exit(1);
            }
            println!("Usage: bla bla bla");
        }
        _ => println!("Unknown subcommand '{}'", subcommand),
    }
}
