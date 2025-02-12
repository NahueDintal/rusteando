use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use pbkdf2::pbkdf2;
use hmac::Hmac;
use sha2::Sha256;
use rand::Rng;
use std::fs;
use std::io::{self, Write};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

const SALT_LENGTH: usize = 16;
const KEY_LENGTH: usize = 32;
const IV_LENGTH: usize = 16;
const PBKDF2_ITERATIONS: u32 = 100_000;

fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_LENGTH] {
    let mut key = [0u8; KEY_LENGTH];
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
    key
}

fn encrypt(data: &[u8], password: &str) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let salt: [u8; SALT_LENGTH] = rng.gen();
    let iv: [u8; IV_LENGTH] = rng.gen();

    let key = derive_key(password, &salt);
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();

    let mut buffer = Vec::new();
    buffer.extend_from_slice(&salt);
    buffer.extend_from_slice(&iv);
    buffer.extend_from_slice(&cipher.encrypt_vec(data));

    buffer
}

fn decrypt(data: &[u8], password: &str) -> Result<Vec<u8>, &'static str> {
    if data.len() < SALT_LENGTH + IV_LENGTH {
        return Err("Datos cifrados inválidos");
    }

    let salt = &data[..SALT_LENGTH];
    let iv = &data[SALT_LENGTH..SALT_LENGTH + IV_LENGTH];
    let ciphertext = &data[SALT_LENGTH + IV_LENGTH..];

    let key = derive_key(password, salt);
    let cipher = Aes256Cbc::new_from_slices(&key, iv).unwrap();

    match cipher.decrypt_vec(ciphertext) {
        Ok(plaintext) => Ok(plaintext),
        Err(_) => Err("Contraseña incorrecta o datos corruptos"),
    }
}

pub fn save_encrypted_data(filename: &str, data: &[u8], password: &str) -> io::Result<()> {
    let encrypted_data = encrypt(data, password);
    let mut file = fs::File::create(filename)?;
    file.write_all(&encrypted_data)?;
    Ok(())
}

pub fn load_encrypted_data(filename: &str, password: &str) -> Result<Vec<u8>, &'static str> {
    let encrypted_data = fs::read(filename).map_err(|_| "No se pudo leer el archivo")?;
    decrypt(&encrypted_data, password)
}
