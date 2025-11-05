use std::env;
use std::path::Path;

pub mod decrypt;
pub mod encrypt;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cryptofiles <encrypt|decrypt> <folder_path> [key]");
        return;
    }

    let command = &args[1];
    let folder_path = Path::new(&args[2]);

    match command.as_str() {
        "encrypt" => match encrypt::encrypt_folder(folder_path) {
            Ok(key) => println!("Folder encrypted successfully. Key: {}", key),
            Err(e) => println!("Error encrypting folder: {}", e),
        },
        "decrypt" => {
            if args.len() < 4 {
                println!("Usage: cryptofiles decrypt <folder_path> <key>");
                return;
            }
            let key = &args[3];
            match decrypt::decrypt_folder(folder_path, key) {
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
    use std::fs::{self, create_dir_all};
    use tempfile::tempdir;

    #[test]
    fn test_encrypt_decrypt() {
        let dir = tempdir().unwrap();
        let folder_path = dir.path();
        let file_path = folder_path.join("test.txt");
        let subfolder_path = folder_path.join("subfolder");
        let subfolder_file = subfolder_path.join("test.txt");

        create_dir_all(&subfolder_path).unwrap();
        fs::write(&file_path, "hello world").unwrap();
        fs::write(&subfolder_file, "subfolder file content").unwrap();

        let key = encrypt::encrypt_folder(folder_path).unwrap();

        decrypt::decrypt_folder(&folder_path, &key).unwrap();

        let decrypted_file_path = folder_path.join("test.txt");
        let decrypted_content = fs::read_to_string(&decrypted_file_path).unwrap();

        let subfolder_file_path = folder_path.join("subfolder/test.txt");
        let subfolder_content = fs::read_to_string(&subfolder_file_path).unwrap();

        assert_eq!(decrypted_content, "hello world");
        assert_eq!(subfolder_content, "subfolder file content");

        fs::remove_dir_all(&dir).unwrap();
    }
}
