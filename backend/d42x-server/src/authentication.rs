use std::time::Duration;

use crate::config;
use chrono::Utc;
use jwt::{Claims, RegisteredClaims, SignWithKey};
use sea_orm::prelude::Uuid;

pub const CLAIM_UID: &str = "UID";
pub const CLAIM_USERNAME: &str = "USERNAME";

const SUBJECT: &str = "user.log_in";

#[derive(Clone, Debug)]
pub struct AuthInformation {
    pub id: Uuid,
    pub username: String,
}

pub fn gen_jwt_token(uid: &Uuid, username: &str) -> String {
    let now = chrono::Utc::now();

    let exp_date_time = now + Duration::from_secs(*config::EXP as u64);

    let mut claims = Claims::new(RegisteredClaims {
        issuer: Some(config::ISS.to_string()),
        subject: Some(SUBJECT.to_owned()),
        audience: Some(config::AUD.to_string()),
        expiration: Some(exp_date_time.timestamp() as u64),
        not_before: Some(now.timestamp() as u64),
        issued_at: Some(now.timestamp() as u64),
        json_web_token_id: Some(Uuid::new_v4().to_string()),
    });

    claims
        .private
        .insert(CLAIM_UID.to_string(), serde_json::json!(uid));
    claims
        .private
        .insert(CLAIM_USERNAME.to_string(), serde_json::json!(username));

    claims.sign_with_key(&*config::JWT_KEY).unwrap()
}

pub fn validate_claims(claims: &Claims) -> bool {
    let now = Utc::now();
    let now_timestamp = now.timestamp() as u64;

    match &claims.registered {
        RegisteredClaims {
            issuer: Some(iss),
            subject: Some(sub),
            audience: Some(aud),
            expiration: Some(exp),
            not_before: Some(nbe),
            issued_at: Some(issat),
            json_web_token_id: Some(_),
        } if *iss == *config::ISS
            && *sub == SUBJECT
            && *aud == *config::AUD
            && *exp > now_timestamp
            && *nbe < now_timestamp
            && *issat < now_timestamp =>
        {
            true
        }
        _ => false,
    }
}
