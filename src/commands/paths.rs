use crate::utils::printing;
use dirs;
use std::fs;
use std::path::PathBuf;

pub fn create_dir(path: &PathBuf) {
    if path.exists() {
        printing::info(format!(
            "A directory already exists at {}.",
            path.to_string_lossy()
        ))
    } else {
        printing::progress(format!("Creating directory at {}.", path.to_string_lossy()));
        match fs::create_dir_all(&path) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem: {:?}", error),
        }
    }
}

pub fn vim_plugins() -> PathBuf {
    home()
        .join(".vim")
        .join("pack")
        .join("plugins")
        .join("start")
}

pub fn repos() -> PathBuf {
    home().join("repos")
}

fn home() -> PathBuf {
    match dirs::home_dir() {
        Some(directory) => directory,
        None => panic!("Cannot find the home directory."),
    }
}
