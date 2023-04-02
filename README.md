# RSA

## Usage

```rust
let private_key = PrivateKey::new().expect("failed to generate a private key");
println!("Private Key: {:?}", private_key);
let public_key = PublicKey::from(&private_key);
println!("Public Key: {:?}", public_key);
let m = BigUint::from_bytes_le(&[42]);
println!("Message: {:?}", m);
let c = public_key.encrypt(&m);
println!("Encrypted message: {:?}", c);
let m_decrypted = private_key.decrypt(&c).expect("failed to decrypt");
println!("Decrypted message: {:?}", m_decrypted);
```
