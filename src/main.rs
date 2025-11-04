use std::env;
use std::path::Path;

pub mod encrypt;
pub mod decrypt;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cryptofiles <encrypt|decrypt> <folder_path> [key] [destination_path]");
        return;
    }

    let command = &args[1];
    let folder_path = Path::new(&args[2]);

    match command.as_str() {
        "encrypt" => {
            match encrypt::encrypt_folder(folder_path) {
                Ok(key) => println!("Folder encrypted successfully. Key: {}", key),
                Err(e) => println!("Error encrypting folder: {}", e),
            }
        }
        "decrypt" => {
            if args.len() < 5 {
                println!("Usage: cryptofiles decrypt <folder_path> <key> <destination_path>");
                return;
            }
            let key = &args[3];
            let destination_path = Path::new(&args[4]);
            match decrypt::decrypt_folder(folder_path, key, destination_path) {
                Ok(_) => println!("Folder decrypted successfully."),
                Err(e) => println!("Error decrypting folder: {}", e),
            }
        }
        _ => {
            println!("Unknown command. Use 'encrypt' or 'decrypt'.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};

    #[test]
    fn test_encrypt_decrypt() {
        let dir = tempdir().unwrap();
        let folder_path = dir.path();
        let file_path = folder_path.join("test.txt");
        fs::write(&file_path, "hello world").unwrap();

        let key = encrypt::encrypt_folder(folder_path).unwrap();

        let mc = new_magic_crypt!(&key, 256);
        let encrypted_folder_name = mc.encrypt_str_to_base64(folder_path.file_name().unwrap().to_str().unwrap());
        let encrypted_folder_path = folder_path.with_file_name(&encrypted_folder_name);

        assert!(encrypted_folder_path.exists());
        assert!(folder_path.join("key.txt").exists());

        let decrypted_dir = tempdir().unwrap();
        let decrypted_destination_path = decrypted_dir.path();

        decrypt::decrypt_folder(&encrypted_folder_path, &key, decrypted_destination_path).unwrap();

        let decrypted_folder_path = decrypted_destination_path.join(folder_path.file_name().unwrap());
        let decrypted_file_path = decrypted_folder_path.join("test.txt");
        let decrypted_content = fs::read_to_string(decrypted_file_path).unwrap();

        assert_eq!(decrypted_content, "hello world");

        fs::remove_dir_all(&encrypted_folder_path).unwrap();
        fs::remove_file(folder_path.join("key.txt")).unwrap();
    }
}