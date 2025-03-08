use std::pin::Pin;

use axum::{
    body::{Body, to_bytes},
    http::{Method, Request, Response, header::CONTENT_TYPE},
};
use lazy_static::lazy_static;
use soft_aes::aes::{AES_BLOCK_SIZE, aes_dec_cbc, aes_enc_cbc};
use tower::{Layer, Service};

const TEXT_PLAIN: &str = "text/plain;charset=UTF-8";
const APPLICATION_JSON: &str = "application/json;charset=UTF-8";

lazy_static! {
    static ref PADDING: Option<&'static str> = Some("PKCS7");
}

#[derive(Clone)]
pub struct CipherLayer {
    aes_key: String,
    aes_iv: [u8; AES_BLOCK_SIZE],
}

impl CipherLayer {
    pub fn new(aes_key: String, aes_iv: [u8; 16]) -> Self {
        Self { aes_key, aes_iv }
    }
}

impl<S> Layer<S> for CipherLayer
where
    S: Clone,
{
    type Service = CipherService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service {
            inner,
            aes_key: self.aes_key.clone(),
            aes_iv: self.aes_iv.clone(),
        }
    }
}

#[derive(Clone)]
pub struct CipherService<S>
where
    S: Clone,
{
    inner: S,
    aes_key: String,
    aes_iv: [u8; AES_BLOCK_SIZE],
}

impl<S> Service<Request<Body>> for CipherService<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + Sync + 'static,
    S::Future: Send,
    S::Error: Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<S::Response, S::Error>> + Send>>;

    #[inline]
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let mut this = self.clone();

        Box::pin(async move {
            let mut need_cipher = false;
            let is_api = request.uri().to_string().starts_with("/api/");
            let is_get = Method::GET == *request.method();

            match *request.method() {
                Method::POST | Method::DELETE | Method::PUT if is_api => {
                    need_cipher = true;
                }
                _ => {}
            }

            let content_type = request.headers().get(CONTENT_TYPE);
            if need_cipher && content_type.is_some() && content_type.unwrap() == TEXT_PLAIN {
                let (mut parts, body) = request.into_parts();

                let body = this.decrypt_body(body).await;

                parts
                    .headers
                    .insert(CONTENT_TYPE, APPLICATION_JSON.parse().unwrap());
                request = Request::from_parts(parts, body);
            }

            match this.inner.call(request).await {
                Ok(mut response) => {
                    if is_get && is_api && !need_cipher {
                        need_cipher = true;
                    }

                    if need_cipher {
                        let (mut parts, body) = response.into_parts();

                        let body = this.encrypt_body(body).await;

                        parts
                            .headers
                            .insert(CONTENT_TYPE, TEXT_PLAIN.parse().unwrap());

                        response = Response::from_parts(parts, body);
                    }

                    Ok(response)
                }
                Err(e) => Err(e),
            }
        })
    }
}

impl<S> CipherService<S>
where
    S: Clone,
{
    async fn decrypt_body(&self, body: Body) -> Body {
        let body = to_bytes(body, usize::MAX).await.unwrap();

        if body.len() == 0 {
            return Body::from(body);
        }

        let body = String::from_utf8(body.to_vec()).unwrap();
        let body = hex::decode(body).expect("decrypt fail, hex_decode fail");

        let de_body = aes_dec_cbc(&body, self.aes_key.as_bytes(), &self.aes_iv, *PADDING)
            .expect("AES decode failed");
        let de_body = String::from_utf8(de_body).unwrap();
        // debug!("decoded body: {}", de_body);

        Body::from(de_body)
    }

    async fn encrypt_body(&self, body: Body) -> Body {
        let body = to_bytes(body, usize::MAX).await.unwrap();

        if body.len() == 0 {
            return Body::from(body);
        }

        let body = String::from_utf8(body.to_vec()).unwrap();

        // debug!("raw body: {}", body);
        let body = aes_enc_cbc(
            body.as_bytes(),
            self.aes_key.as_bytes(),
            &self.aes_iv,
            *PADDING,
        )
        .expect("body encrypted failed");
        let body = hex::encode(body);

        Body::from(body)
    }
}
