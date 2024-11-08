/* 
    Advanced Encryption Standard AES, 一种对成加密的算法,用同一个密钥进行加解密
*/

// use ring::aead::{Aad, LessSafeKey, UnboundKey, Nonce, AES_256_GCM};
// use ring::rand::SystemRandom;

// fn encrypt_aes_gcm(key: &[u8], nonce: &[u8], data: &mut Vec<u8>, aad: &[u8]) {
//     let key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, key).unwrap());
//     let nonce = Nonce::assume_unique_for_key(nonce.try_into().unwrap());
//     key.seal_in_place_append_tag(nonce, Aad::from(aad), data).unwrap();
// }

// fn decrypt_aes_gcm(key: &[u8], nonce: &[u8], data: &mut Vec<u8>, aad: &[u8]) -> Vec<u8> {
//     let key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, key).unwrap());
//     let nonce = Nonce::assume_unique_for_key(nonce.try_into().unwrap());
//     let decrypted_data = key.open_in_place(nonce, Aad::from(aad), data).unwrap();
//     decrypted_data.to_vec() 
// }

fn main() {
    // let key = [0u8; 32]; 
    // let nonce = [0u8; 12]; 
    // let aad = b"extra data"; 
    // let mut data = b"secret message".to_vec(); 

    // // 加密
    // encrypt_aes_gcm(&key, &nonce, &mut data, aad);
    // println!("Encrypted data: {:?}", data);

    // // 解密
    // let decrypted_data = decrypt_aes_gcm(&key, &nonce, &mut data, aad);
    // println!("Decrypted message: {:?}", String::from_utf8(decrypted_data.to_vec()).unwrap());
}
