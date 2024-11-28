use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_wordlist(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let words = reader.lines().map_while(Result::ok).collect();
    Ok(words)
}

fn crack_md5(hash_to_match: &str, wordlist: Vec<String>) -> Option<String> {
    for word in wordlist {
        let hash = format!("{:x}", md5::compute(word.as_bytes()));
        if hash == hash_to_match {
            return Some(word);
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter The MD5 Hash");
    let mut hash_to_match = String::new();
    io::stdin().read_line(&mut hash_to_match)?;
    let hash_to_match = hash_to_match.trim();

    println!("Enter The Path To The Wordlist");
    let mut wordlist_path = String::new();
    io::stdin().read_line(&mut wordlist_path)?;
    let wordlist_path = wordlist_path.trim();

    if !Path::new(wordlist_path).exists() {
        eprintln!("The Specified File Does Not Exist:{}", wordlist_path);
        return Err("File Not Found".into());
    }

    let wordlist = read_wordlist(wordlist_path)?;

    println!("Attempting To Crack The Hash...");
    match crack_md5(hash_to_match, wordlist) {
        Some(password) => println!("Password Found!:{}", password),
        None => println!("Password Not Found In The Wordlist."),
    }
    Ok(())
}
