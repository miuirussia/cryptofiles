use base64::{engine::general_purpose, Engine as _};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::Rng;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};
use walkdir::WalkDir;

pub fn encrypt_folder(folder_path: &Path) -> Result<String, io::Error> {
    let mut key = [0u8; 32];
    rand::thread_rng().fill(&mut key);
    let key_b64 = general_purpose::STANDARD.encode(&key);
    let crypt = new_magic_crypt!(&key_b64, 256);

    let key_path = folder_path.join("key.txt");
    fs::write(key_path, &key_b64)?;

    let mut files_to_encrypt = Vec::new();
    for entry in WalkDir::new(folder_path) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap() != "key.txt" {
            files_to_encrypt.push(path.to_path_buf());
        }
    }

    for file_path in files_to_encrypt {
        let relative_path = file_path.strip_prefix(folder_path).unwrap();
        let encrypted_path_b64 = crypt.encrypt_str_to_base64(relative_path.to_str().unwrap());

        let safe_b64_path = encrypted_path_b64
            .replace("/", "_")
            .replace("+", "-")
            .replace("=", "");

        let new_path = folder_path.join(&safe_b64_path);

        let mut file_content = Vec::new();
        File::open(&file_path)?.read_to_end(&mut file_content)?;

        let encrypted_content = crypt.encrypt_bytes_to_bytes(&file_content);

        let mut new_file = File::create(&new_path)?;
        new_file.write_all(&encrypted_content)?;

        fs::remove_file(&file_path)?;

        let mut current_dir = file_path.parent().unwrap();
        while current_dir != folder_path {
            if fs::read_dir(current_dir)?.next().is_none() {
                fs::remove_dir(current_dir)?;
                current_dir = current_dir.parent().unwrap();
            } else {
                break;
            }
        }
    }

    Ok(key_b64)
}