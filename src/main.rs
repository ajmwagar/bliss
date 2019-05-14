use std::error::Error;
use bliss::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut bliss = Bliss::new();

    // let langs = bliss.supported_langs()?;

    // println!("Supported Languages:\n========================\n{}", langs.join(", "));

    let ignore = bliss.get_lang_gitignore("rust");

    print!("{}", ignore.unwrap());

    // let is_supported = bliss.is_supported("rust");

    // println!("{}", is_supported);

    // Save cache before exiting
    bliss.cache.save()?;

    Ok(())
}

