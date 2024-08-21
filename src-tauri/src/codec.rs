use base64::prelude::*;
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};

pub fn rsa_encode(pem: &str, secret: &str) -> String {
    let mut rng = rand::thread_rng();
    let public_key = RsaPublicKey::from_public_key_pem(pem).unwrap();
    let enc_data = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, secret.as_bytes())
        .expect("failed to encrypt");

    base_encode(enc_data)
}

fn base_encode<T: AsRef<[u8]>>(input: T) -> String {
    BASE64_STANDARD.encode(input)
}

mod tests {
    #[test]
    fn rsa_encode_works() {
        use super::rsa_encode;
        let pem = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAo4QBxWqrnzFAkCLBZ/+z
UGZPrbV267z/2fItMD91nZa79TqAmM0SjHCe+ESq9YbRAnQXTXDOXJc34Z9a2m9y
ZaBWexHPprIygKm1PIi9UrVa58EV/AbiBRc53ExvRDVZDjG6OPZfceTJS4nA+hRR
idT9ZlACtXid++lw2/Y32woJRj40Mjaxa0Hi7C0A+vyVL8SvDh1AvFOW5/dGnKkf
WMelpsyjmnJ0Ub1zr46aDT1m9Rb/lBijLjOqeEt0FgvpXJM5mb8N0oWdLoxir4MX
Z+MVhfGZtKOu533fwCvYD35Br/LbBLxnTwPolrvLZKOS6wEktWVqx/bJMc20h87G
8wIDAQAB
-----END PUBLIC KEY-----";
        let _ = rsa_encode(pem, "secret");
    }
}
