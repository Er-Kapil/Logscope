mod analyze;
mod input;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
    }
}

fn run() -> Result<(), String> {
    let args = input::take_input()?;
    let (length, specs) = analyze::analyser(&args.filepath)?;
    println!("Argument 1: {}", args.app);
    println!("Argument 2: {}", args.filepath.into_string().unwrap());
    println!();
    println!("Total log entries : {length}");
    println!();
    println!("Log levels");
    for (key, val) in specs.iter() {
        println!("  {} : {}", key, val);
    }
    return Ok(());
}
