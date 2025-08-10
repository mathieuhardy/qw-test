mod error;

use crate::error::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err(Error::MissingInpputFile);
    }

    println!("{}", args[1]);

    Ok(())
}
