use super::*;

use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};
use rsa::{PublicKey, RsaPublicKey};

#[wasm_bindgen]
#[derive(Debug)]
pub struct RsaPublic {
    pub_instance: RsaPublicKey,
    pub_pem: String,
}

#[wasm_bindgen]
impl RsaPublic {
    #[wasm_bindgen(constructor)]
    // TODO: is n and e needed in struct?
    pub fn new(input_key_pem: String) -> Self {
        utils::set_panic_hook();
        let pub_instance = RsaPublicKey::from_public_key_pem(&input_key_pem)
            .expect("Failed to read public key bytes!");
        let pub_pem = pub_instance
            .to_public_key_pem(LineEnding::default())
            .expect("Failed to transform key to pem!");

        Self {
            pub_instance,
            pub_pem,
        }
    }

    // TODO: add custom digest algo support for OAEP; now default is Sha256
    pub fn encrypt(&self, msg: &[u8], padding_scheme: &str) -> Vec<u8> {
        // default padding scheme: OAEP with sha256
        let padding = match padding_scheme {
            "PKCS1V15Encrypt" => PaddingScheme::new_pkcs1v15_encrypt(),
            "OAEP" => PaddingScheme::new_oaep::<sha2::Sha256>(),
            _ => PaddingScheme::new_oaep::<sha2::Sha256>(),
        };

        let mut rng = rand::thread_rng();

        self.pub_instance
            .encrypt(&mut rng, padding, &msg[..])
            .expect("Failed to encrypt the message!")
    }

    // TODO add custom hasher input
    pub fn verify(&self, digest: &[u8], sig: Vec<u8>, padding_str: &str) -> bool {
        let padding = match padding_str {
            // use sha256 as default hasher for pkcs1v15
            // TODO: add mapping for hash function. The input of hash here is to check the digest length
            "PKCS1V15Sign" => PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA3_256)),
            "PSS" => PaddingScheme::new_pss_with_salt::<sha2::Sha256, ThreadRng>(rand::thread_rng(), digest.len()),
            _ => PaddingScheme::new_pss_with_salt::<sha2::Sha256, ThreadRng>(rand::thread_rng(), digest.len())
        };
        self.pub_instance.verify(padding, digest, &sig).is_ok()
    }

    #[wasm_bindgen(js_name = getPublicKeyContent)]
    pub fn get_public_key_content(&self, fmt: &str) -> JsValue {
        match fmt {
            "pem" => JsValue::from_str(&self.pub_pem),
            // TODO: Der generated by this func cannnot be verified by OpenSSL
            "der" => {
                serde_wasm_bindgen::to_value(
                    &self.pub_instance
                        .to_public_key_der()
                        .expect("Failed to transform public key to der!")
                        .to_der()
                        .to_vec()
                ).unwrap()
            },
            _ => panic!("Only pem and der supported")
        }
    }
}

#[cfg(test)]
mod rsa_public_tests {
    use super::*;
    use super::super::rsa_private::RsaPrivate;

    #[test]
    fn can_new_with_rsa_private() {
        let rsa_public = RsaPublic::new(RsaPrivate::new(Some(1024), None).get_public_key_pem());
        assert_eq!(rsa_public.pub_pem.is_empty(), false);
    }

    #[test]
    #[should_panic]
    fn cannot_new_with_empty_key() {
        RsaPublic::new(String::from(""));
    }

    #[test]
    fn can_encrypt() {
        let rsa_pubic = RsaPublic::new(RsaPrivate::new(Some(1024), None).get_public_key_pem());
        assert_ne!(rsa_pubic.encrypt(b"secret", ""), Vec::<u8>::new());
    }
}