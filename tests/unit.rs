use bliss::*;

fn setup() -> Bliss {
    Bliss::new()
}

#[test]
/// Download supported_languages from the web
fn web_supported_languages() {
    let bliss = setup();

    let result = bliss.supported_langs();

}

#[test]
/// Test pulling get_lang_gitignore with the rust language
fn web_gitignore_rust() {
    let mut bliss = setup();

    let result = bliss.get_lang_gitignore("rust");

    let sucess = match result {
        None => false,
        Some(_) => true
    };

    assert!(sucess);
}
