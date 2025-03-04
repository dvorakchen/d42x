use hmac::Mac;
use lazy_static::lazy_static;
use sea_orm::prelude::Uuid;
use serde::Serialize;
use sha2::Sha256;
use soft_aes::aes::AES_BLOCK_SIZE;

lazy_static! {
    pub static ref KEY: String = dotenv::var("AES_KEY").expect("not found AES_KEY");
    pub static ref IV: [u8; 16] = {
        let key = dotenv::var("AES_IV").expect("not found AES_IV");
        if key.len() != AES_BLOCK_SIZE {
            panic!("Wrong AES_IV: {}", key);
        }

        let mut res = [0u8; 16];
        res.copy_from_slice(key.as_bytes());
        res
    };
    pub static ref DATABASE_URL: String =
        dotenv::var("DATABASE_URL").expect("not found DATABASE_URL");
    pub static ref ADDRESS: String = dotenv::var("ADDRESS").expect("not found ADDRESS");
    pub static ref CORS: String = dotenv::var("CORS").expect("not found CORS");
    static ref _JWT_KEY: Uuid = Uuid::new_v4();
    pub static ref JWT_KEY: hmac::Hmac<Sha256> =
        hmac::Hmac::new_from_slice(_JWT_KEY.as_bytes()).unwrap();
    pub static ref ISS: String = dotenv::var("ISS").expect("not found ISS");
    pub static ref AUD: String = dotenv::var("AUD").expect("not found AUD");
    pub static ref EXP: usize = dotenv::var("EXP").expect("not found EXP").parse().unwrap();
}

#[derive(Serialize, Debug)]
pub enum AllowMemeFormats {
    JPG,
    JPEG,
    PNG,
    GIF,
    WEBP,
    WEBM,
}

impl TryFrom<&str> for AllowMemeFormats {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_ref() {
            "jpg" => Ok(Self::JPG),
            "jpeg" => Ok(Self::JPEG),
            "png" => Ok(Self::PNG),
            "gif" => Ok(Self::GIF),
            "webp" => Ok(Self::WEBP),
            "webm" => Ok(Self::WEBM),
            _ => Err(format!("unsupport format: {}", value)),
        }
    }
}
