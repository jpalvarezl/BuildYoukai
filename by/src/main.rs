mod runtime;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, world!");
    let runtime = runtime::parse_from_cli();

    println!("Runtime: {:?}", runtime);
    Ok(())
}
