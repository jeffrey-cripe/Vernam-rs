use bytes::Bytes;
use std::str;
use rand::Rng;
use std::io;

fn main() {
    println!("Please enter what you would like to be encrypted... (Enter when done)\n\n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("\nVernam Cipher (One Time Pad):");

    let plaintext = input.trim();
    let random_string = random(plaintext.len());
    
    println!("Secret Key: {:?}\n", random_string);

    let ciphertext = vernam(plaintext.to_string(), random_string.to_string());

    println!("\nCiphertext: {:?}\n", ciphertext);

    println!("\nPlaintext: {:?}\n", vernam(ciphertext, random_string.to_string()));
}

fn vernam(text: String, key: String) -> String {
    let text_space = Bytes::from(text);
    let key_space = Bytes::from(key);
    let mut cipher_space: Vec<u8> = Vec::new();

    for index in 0..text_space.len() {
        cipher_space.push(text_space[index] ^ key_space[index]);
    }

    let s = match str::from_utf8(&cipher_space) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    s.to_string()
}

fn random(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut rand_array: Vec<u8> = Vec::new();

    for _ in 0..length {
        let mut rand_val: u8;

        loop {
            rand_val = rng.gen();
            if rand_val >= 32 && rand_val <= 126 { break; }
        }
        
        rand_array.push(rand_val);
    }

    let s = match str::from_utf8(&rand_array) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    s.to_string()
}