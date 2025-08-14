use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        return Err("not enough arguments".into());
    }
    let n1: u128 = args[1].as_str().parse()?;
    let n2: u128 = args[2].as_str().parse()?;
    println!("{}", percona_data_federation_server::compute(n1, n2));
    Ok(())
}
