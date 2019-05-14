use clap::{Arg, App, SubCommand};

use std::error::Error;
use bliss::*;

fn main() -> Result<(), Box<dyn Error>> {
     let matches = App::new("bliss")
                          .version("1.0")
                          .author("Avery Wagar <ajmw.subs@gmail.com>")
                          .about("Ignorance is bliss! Ignore your .gitignore")
                          .arg(Arg::with_name("LANGUAGE")
                               .help("Sets the input file to use")
                               .required(false)
                               .index(1))
                          .subcommand(SubCommand::with_name("list")
                                      .about("list supported languages"))
                          .subcommand(SubCommand::with_name("cache")
                                      .about("manage bliss cache")
                                      .subcommand(SubCommand::with_name("clear").about("clear bliss cache"))
                                      .subcommand(SubCommand::with_name("update").about("update bliss cache")))
                          .get_matches();


    let mut bliss = Bliss::new();

    if let Some(_) = matches.subcommand_matches("list") {
        let langs = bliss.supported_langs()?;

        println!("Supported Languages:\n========================\n{}", langs.join(", "));
    }
    // Cache modification
    else if let Some(matches) = matches.subcommand_matches("cache") {
        // Clear cache
        if let Some(matches) = matches.subcommand_matches("clear") {
            // TODO Clear cache
            println!("Clearing cache...");

            std::fs::remove_dir_all(format!("{}/bliss", dirs::cache_dir().unwrap().to_string_lossy()))?;

            // Exit early
            std::process::exit(0);
        }
        // Update cache
        else if let Some(matches) = matches.subcommand_matches("update") {
            println!("Updating cache...");

            // Update supported languages
            bliss.supported_langs()?;

            // Update .gitignore templates
            for lang in bliss.cache.gitignores.clone().keys() {
                bliss.get_lang_gitignore(lang);
            }
        }
    }
    else {
        for lang in matches.value_of("LANGUAGE").unwrap_or("").split(" ").collect::<Vec<_>>().join("").split(",") {
            if bliss.is_supported(lang){
                let ignore = bliss.get_lang_gitignore(lang); 
                print!("{}", ignore.unwrap());
            }
            // else {
            //     // TODO Don't print other languages if one fails
            //     eprintln!("{} is not a supported language", lang);
            //     break;
            // }
        }
    }


    // let is_supported = bliss.is_supported("rust");

    // println!("{}", is_supported);

    // Save cache before exiting
    bliss.cache.save()?;

    Ok(())
}

