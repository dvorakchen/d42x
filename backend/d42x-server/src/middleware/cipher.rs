use crate::config;
use axum::{
    body::{Body, to_bytes},
    extract::Request,
    http::{Method, header::CONTENT_TYPE},
    middleware::Next,
    response::Response,
};
use lazy_static::lazy_static;
use soft_aes::aes::{aes_dec_cbc, aes_enc_cbc};
use tracing::debug;

const TEXT_PLAIN: &str = "text/plain;charset=UTF-8";
const APPLICATION_JSON: &str = "application/json;charset=UTF-8";

lazy_static! {
    static ref PADDING: Option<&'static str> = Some("PKCS7");
}

pub async fn cipher_middleware(mut request: Request, next: Next) -> Response {
    let mut need_cipher = false;
    let is_admin_api = request.uri().to_string().starts_with("/api/admin/");
    let is_get = Method::GET == *request.method();

    match *request.method() {
        Method::POST | Method::DELETE | Method::PUT if is_admin_api => {
            need_cipher = true;
        }
        _ => {}
    }

    // let need_cipher = request.uri().to_string().starts_with("/api/");
    let content_type = request.headers().get(CONTENT_TYPE);

    if need_cipher && content_type.is_some() && content_type.unwrap() == TEXT_PLAIN {
        let (mut parts, body) = request.into_parts();

        let body = decrypt_body(body).await;

        parts
            .headers
            .insert(CONTENT_TYPE, APPLICATION_JSON.parse().unwrap());
        request = Request::from_parts(parts, body);
    }

    let mut response = next.run(request).await;

    if is_get && is_admin_api && !need_cipher {
        need_cipher = true;
    }

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

    if body.len() == 0 {
        return Body::from(body);
    }

    let body = String::from_utf8(body.to_vec()).unwrap();
    debug!("encrypted body: {}", body);
    let body = hex::decode(body).expect("decrypt fail, hex_decode fail");

    let de_body = aes_dec_cbc(&body, config::KEY.as_bytes(), &config::IV, *PADDING)
        .expect("AES decode failed");
    let de_body = String::from_utf8(de_body).unwrap();
    debug!("decoded body: {}", de_body);

    Body::from(de_body)
}

async fn encrypt_body(body: Body) -> Body {
    let body = to_bytes(body, usize::MAX).await.unwrap();

    debug!("len: {}", body.len());
    if body.len() == 0 {
        return Body::from(body);
    }

    let body = String::from_utf8(body.to_vec()).unwrap();
    debug!("body: {}", body);

    debug!("raw body: {}", body);
    let body = aes_enc_cbc(
        body.as_bytes(),
        config::KEY.as_bytes(),
        &config::IV,
        *PADDING,
    )
    .expect("msg");
    let body = hex::encode(body);
    debug!("encoded body: {}", body);

    Body::from(body)
}
