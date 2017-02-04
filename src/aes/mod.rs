use crypto::aes::{self, KeySize};
use crypto::{blockmodes, buffer, symmetriccipher};
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use crypto::digest::Digest;
use crypto::symmetriccipher::SynchronousStreamCipher;
use rand::Rng;
use rand::os::OsRng;
use serialize::base64::{STANDARD, ToBase64, FromBase64};
use std::iter::repeat;

pub fn GenerateKey() -> Vec<u8> {
    let mut gen = OsRng::new().expect("Failed to get OS random generator");

    let mut key: Vec<u8> = repeat(0u8).take(16).collect();
    gen.fill_bytes(&mut key[..]);

    println!("Key: {}", key.to_base64(STANDARD));

    return key;
}

pub fn GenerateNonce() -> Vec<u8> {
    let mut gen = OsRng::new().expect("Failed to get OS random generator");

    let mut nonce: Vec<u8> = repeat(0u8).take(16).collect();
    gen.fill_bytes(&mut nonce[..]);

    println!("Nonce: {}", nonce.to_base64(STANDARD));

    return nonce;
}

pub fn AESCipher(data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {


    let mut cipher = aes::cbc_encryptor(KeySize::KeySize256, &key, &nonce, blockmodes::PkcsPadding);

    //data.as_bytes();

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    //let secret = "I like Nickelback";

    loop {
        let result = try!(cipher.encrypt(&mut read_buffer, &mut write_buffer, true));

        // "write_buffer.take_read_buffer().take_remaining()" means:
        // from the writable buffer, create a new readable buffer which
        // contains all data that has been written, and then access all
        // of that data as a slice.
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    println!("Ciphertext: {}", final_result.to_base64(STANDARD));

    Ok(final_result)


}

pub fn AESDecrypt(encrypted_data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            key,
            nonce,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    println!("Decrypted Text: {:?}", String::from_utf8_lossy(&final_result));

    Ok(final_result)
}
