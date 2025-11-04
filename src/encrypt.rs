use std::fs;
use std::path::Path;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand;
use base64::{Engine as _, engine::general_purpose};

pub fn encrypt_folder(folder_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let key = generate_key();
    let mc = new_magic_crypt!(&key, 256);

    let encrypted_folder_name = mc.encrypt_str_to_base64(folder_path.file_name().unwrap().to_str().unwrap());
    let encrypted_folder_path = folder_path.with_file_name(encrypted_folder_name);
    fs::create_dir_all(&encrypted_folder_path)?;

    encrypt_directory_contents(folder_path, &encrypted_folder_path, &mc)?;

    fs::write(folder_path.join("key.txt"), &key)?;

    Ok(key)
}

fn encrypt_directory_contents(
    dir: &Path,
    encrypted_dir: &Path,
    mc: &magic_crypt::MagicCrypt256,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(dir).unwrap();
        let encrypted_relative_path = mc.encrypt_str_to_base64(relative_path.to_str().unwrap());
        let encrypted_path = encrypted_dir.join(encrypted_relative_path);

        if path.is_dir() {
            fs::create_dir_all(&encrypted_path)?;
            encrypt_directory_contents(&path, &encrypted_path, mc)?;
        } else {
            let content = fs::read(&path)?;
            let encrypted_content = mc.encrypt_bytes_to_base64(&content);
            fs::write(&encrypted_path, encrypted_content)?;
        }
    }
    Ok(())
}

fn generate_key() -> String {
    let key: [u8; 32] = rand::random();
    general_purpose::STANDARD.encode(key)
}