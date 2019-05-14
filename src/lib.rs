use std::error::Error;
use std::collections::HashMap;

// Cache managment
use std::fs::{File, create_dir_all};
use std::io::prelude::*;

/// Filename for list of supported languages
const CACHE_LANGS_FILE: &str = "bliss_langs";

/// Bliss, gitignore client
pub struct Bliss {
    /// Cache of gitignore info
    pub cache: Cache,
}

impl Bliss {
    /// Create a new bliss client
    pub fn new() -> Self {

        let cache = match Cache::from() {
            Ok(cache) => cache,
            Err(_e) => Cache::new()
        };

        Bliss {
          cache
        }


    }

    /// Get list of supported languages from cache or fallback to web
    pub fn supported_langs(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let sl = self.cache.supported_langs.clone().unwrap();


        if sl.len() == 0 {
            return Ok(Bliss::get_lang_list()?);
        }
        else {
            return Ok(sl);
        }

    }



    /// Get list of supported languages from web
    pub fn get_lang_list() -> Result<Vec<String>, Box<dyn Error>> {
        let url = format!("https://www.gitignore.io/api/list");

        // TODO: Error handling
        let contents = reqwest::get(&url)?.text()?;

        let contents = contents.replace("\"", "");
        let contents = contents.replace("\n", ",");

        Ok(contents.split(",").map(|string| string.to_string()).collect::<Vec<String>>())

    }

    /// Check whether a given language is supported
    pub fn is_supported(&self, lang: &str) -> bool {
        // TODO Use good result
        let langs = self.supported_langs().unwrap();

        if langs.contains(&lang.to_string()) { true } else { false }
    } 

    /// Get the respective `.gitignore` for a given language
    pub fn get_lang_gitignore(&mut self, lang: &str) -> Option<Gitignore> {
        if self.cache.gitignores.contains_key(lang) {
            return Some(self.cache.gitignores.get(lang).unwrap().clone().to_owned())
        }
        else {


        let url = format!("https://www.gitignore.io/api/{}", lang);

        // TODO: Error handling
        let contents = reqwest::get(&url).unwrap().text().unwrap();
        if contents.contains("undefined") {
            return None;
        }

        // Some(contents);

        // println!("{}", contents);

        let gi = Gitignore { ignored_paths: contents.split("\n").map(|x| x.to_string()).collect() };

        self.cache.gitignores.insert(lang.to_string(), gi.clone());

        Some(gi)
        }
    }
}

/// A gitignore
#[derive(Clone)]
pub struct Gitignore {
    /// Paths to ignore
    ignored_paths: Vec<String>,
}

impl std::fmt::Display for Gitignore {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>{
        writeln!(formatter, "{}", self.ignored_paths.join("\n"))?;

        Ok(())
    }
}

/// Cache of gitignore and language information
pub struct Cache {
    /// Supported languages
    supported_langs: Option<Vec<String>>,
    /// List of gitignores stored in cache
    gitignores: HashMap<String, Gitignore>

}

impl Cache {
    /// Create a new cache
    pub fn new() -> Self {
        Cache {
            supported_langs: Some(Bliss::get_lang_list().unwrap()),
            gitignores: HashMap::new()
        }
    }

    /// Save cache to fs
    #[cfg(unix)]
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        // Get cache
        let cache = dirs::cache_dir().unwrap(); 

        // Setup filesystem 
        let mut path = String::new();

        path.push_str(&cache.to_string_lossy());
        path.push_str(&"/bliss/");

        create_dir_all(format!("{}{}", path, "ignores"))?;

        // Write supported languages
        let mut lang_list = File::create(format!("{}{}", path, CACHE_LANGS_FILE))?;

        lang_list.write_all(self.supported_langs.clone().unwrap().join("\n").as_bytes())?;

        // TODO use rayon
        // Save gitignore templates
        self.gitignores.iter().for_each(|(lang, ignore)| {
            //TODO Correct error handling
            let mut file = File::create(format!("{}ignores/{}.gitignore", path, lang)).unwrap();

            file.write_all(ignore.ignored_paths.join("\n").as_bytes());
        });

        Ok(())

    }

    /// Read Cache from fs
    #[cfg(unix)]
    pub fn from() -> Result<Self, Box<dyn Error>> {
        // Get cache
        let cache = dirs::cache_dir().unwrap(); 

        // Setup filesystem 
        let mut path = String::new();

        path.push_str(&cache.to_string_lossy());
        path.push_str(&"/bliss/");


        // Read supported languages
        let mut supported_langs = String::new();

        File::open(format!("{}{}", path, CACHE_LANGS_FILE))?.read_to_string(&mut supported_langs)?;

        // Read gitignore templates

        Ok(Cache {
            supported_langs: Some(supported_langs.split("\n")
                                 .map(|x| x.to_string())
                                 .collect()),
            gitignores: HashMap::new(),
        })
    }
}
