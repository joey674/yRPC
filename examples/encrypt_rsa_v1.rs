
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;
use num_bigint::BigUint;

fn main() {
    // Generate 2048-bit RSA keys
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    // Prepare a message for encryption
    let message = b"Hello, RSA!";
    let padding = PaddingScheme::new_pkcs1v15_encrypt();

    // Encrypt the message using the public key
    let encrypted_message = public_key.encrypt(&mut rng, padding, &message[..])
        .expect("failed to encrypt");
    println!("Encrypted message: {:?}", encrypted_message);

    // Decrypt the message using the private key
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let decrypted_message = private_key.decrypt(padding, &encrypted_message)
        .expect("failed to decrypt");
    println!("Decrypted message: {:?}", String::from_utf8(decrypted_message).unwrap());
}
