use crypto::aes::{self, KeySize};
use crypto::digest::Digest;
use crypto::symmetriccipher::SynchronousStreamCipher;
use rand::Rng;
use rand::os::OsRng;
use serialize::base64::{STANDARD, ToBase64};
use std::iter::repeat;

pub fn AESCipher(){
    let mut gen = OsRng::new().expect("Failed to get OS random generator");
    let mut key: Vec<u8> = repeat(0u8).take(16).collect();
    gen.fill_bytes(&mut key[..]);
    let mut nonce: Vec<u8> = repeat(0u8).take(16).collect();
    gen.fill_bytes(&mut nonce[..]);
    println!("Key: {}", key.to_base64(STANDARD));
    println!("Nonce: {}", nonce.to_base64(STANDARD));

    let mut cipher = aes::ctr(KeySize::KeySize128, &key, &nonce);
    let secret = "I like Nickelback";
    let mut output: Vec<u8> = repeat(0u8).take(secret.len()).collect();
    cipher.process(secret.as_bytes(), &mut output[..]);
    println!("Ciphertext: {}", output.to_base64(STANDARD));
}
