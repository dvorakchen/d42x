use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::{header::CONTENT_TYPE, Method},
    middleware::Next,
    response::Response,
};
use lazy_static::lazy_static;
use soft_aes::aes::{aes_dec_cbc, aes_enc_cbc, AES_BLOCK_SIZE};
use tracing::debug;

const TEXT_PLAIN: &str = "text/plain;charset=UTF-8";
const APPLICATION_JSON: &str = "application/json;charset=UTF-8";

lazy_static! {
    static ref KEY: String = dotenv::var("AES_KEY").expect("not found AES_KEY");
    static ref IV: [u8; 16] = {
        let key = dotenv::var("AES_IV").expect("not found AES_IV");
        if key.len() != AES_BLOCK_SIZE {
            panic!("Wrong AES_IV: {}", key);
        }

        let mut res = [0u8; 16];
        res.copy_from_slice(key.as_bytes());
        res
    };
    static ref PADDING: Option<&'static str> = Some("PKCS7");
}

pub async fn cipher_middleware(mut request: Request, next: Next) -> Response {
    match *request.method() {
        Method::GET | Method::POST | Method::DELETE | Method::PUT => {}
        _ => return next.run(request).await,
    }

    let need_cipher = request.uri().to_string().starts_with("/api/");
    let content_type = request.headers().get(CONTENT_TYPE).unwrap();

    if need_cipher && content_type == TEXT_PLAIN {
        let (mut parts, body) = request.into_parts();

        let body = decrypt_body(body).await;

        parts
            .headers
            .insert(CONTENT_TYPE, APPLICATION_JSON.parse().unwrap());
        request = Request::from_parts(parts, body);
    }

    let mut response = next.run(request).await;

    if need_cipher {
        let (mut parts, body) = response.into_parts();

        let body = encrypt_body(body).await;

        parts
            .headers
            .insert(CONTENT_TYPE, TEXT_PLAIN.parse().unwrap());

        response = Response::from_parts(parts, body);
    }

    response
}

async fn decrypt_body(body: Body) -> Body {
    let body = to_bytes(body, usize::MAX).await.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();
    debug!("encrypted body: {}", body);
    let body = hex::decode(body).expect("decrypt fail, hex_decode fail");

    let de_body = aes_dec_cbc(&body, KEY.as_bytes(), &IV, *PADDING).expect("AES decode failed");
    let de_body = String::from_utf8(de_body).unwrap();
    debug!("decoded body: {}", de_body);

    Body::from(de_body)
}

async fn encrypt_body(body: Body) -> Body {
    let body = to_bytes(body, usize::MAX).await.unwrap();
    let body = String::from_utf8(body.to_vec()).unwrap();

    debug!("raw body: {}", body);
    let body = aes_enc_cbc(body.as_bytes(), KEY.as_bytes(), &IV, *PADDING).expect("msg");
    let body = hex::encode(body);
    debug!("encoded body: {}", body);

    Body::from(body)
}
