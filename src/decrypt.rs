use std::fs;
use std::path::Path;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

pub fn decrypt_folder(encrypted_folder_path: &Path, key: &str, destination_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mc = new_magic_crypt!(key, 256);

    let decrypted_folder_name_b64 = encrypted_folder_path.file_name().unwrap().to_str().unwrap();
    let decrypted_folder_name = mc.decrypt_base64_to_string(decrypted_folder_name_b64)?;
    let decrypted_folder_path = destination_path.join(decrypted_folder_name);
    fs::create_dir_all(&decrypted_folder_path)?;

    decrypt_directory_contents(encrypted_folder_path, &decrypted_folder_path, &mc)?;

    Ok(())
}

fn decrypt_directory_contents(
    encrypted_dir: &Path,
    decrypted_dir: &Path,
    mc: &magic_crypt::MagicCrypt256,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(encrypted_dir)? {
        let entry = entry?;
        let path = entry.path();
        let encrypted_relative_path_b64 = path.file_name().unwrap().to_str().unwrap();
        let decrypted_relative_path = mc.decrypt_base64_to_string(encrypted_relative_path_b64)?;
        let decrypted_path = decrypted_dir.join(decrypted_relative_path);

        if path.is_dir() {
            fs::create_dir_all(&decrypted_path)?;
            decrypt_directory_contents(&path, &decrypted_path, mc)?;
        } else {
            let content = fs::read_to_string(&path)?;
            let decrypted_content = mc.decrypt_base64_to_bytes(&content)?;
            fs::write(&decrypted_path, decrypted_content)?;
        }
    }
    Ok(())
}