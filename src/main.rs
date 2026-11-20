mod input;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
    }
}

fn run() -> Result<(), String> {
    let args = input::take_input()?;
    println!("Argument 1: {}", args.app);
    println!("Argument 2: {}", args.filepath.into_string().unwrap());
    return Ok(());
}
