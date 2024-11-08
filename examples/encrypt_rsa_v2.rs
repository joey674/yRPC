use rand::rngs::OsRng;
use rpc_framework::*;

fn main() {
    // Generate RSA keys
    let bits = 256;
    let (public_key, private_key) = generate_keys(bits);


    // Prepare a message for encryption
    let message = b"[this is secret message.......]";

    let cyphertext = public_key.encrypt(&message[..])
        .expect("failed to encrypt");
    println!("Encrypted message: {:?}", cyphertext);

    let plaintext = private_key.decrypt(&cyphertext)
        .expect("failed to decrypt");
    println!("Decrypted message: {:?}", String::from_utf8(plaintext).unwrap());
}
