use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

pub fn decrypt_folder(folder_path: &Path, key: &str) -> Result<(), io::Error> {
    let crypt = new_magic_crypt!(key, 256);
    let mut files_to_decrypt = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap() != "key.txt" {
            files_to_decrypt.push(path);
        }
    }

    for file_path in files_to_decrypt {
        let encrypted_path_b64 = file_path.file_name().unwrap().to_str().unwrap();
        let mut original_b64_path = encrypted_path_b64.replace("_", "/").replace("-", "+");

        while original_b64_path.len() % 4 != 0 {
            original_b64_path.push('=');
        }

        let decrypted_path_str = crypt.decrypt_base64_to_string(&original_b64_path).unwrap();
        let relative_path = PathBuf::from(decrypted_path_str);
        let new_path = folder_path.join(relative_path);

        if let Some(parent) = new_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut encrypted_content = Vec::new();
        File::open(&file_path)?.read_to_end(&mut encrypted_content)?;

        let decrypted_content = crypt.decrypt_bytes_to_bytes(&encrypted_content).unwrap();

        let mut new_file = File::create(&new_path)?;
        new_file.write_all(&decrypted_content)?;

        fs::remove_file(&file_path)?;
    }

    let key_path = folder_path.join("key.txt");
    if key_path.exists() {
        fs::remove_file(key_path)?;
    }

    Ok(())
}