use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateSuggestReq {
    pub meme_id: Uuid,
    #[validate(length(min = 1, code = "list should not be 0"))]
    pub list: Vec<String>,
    pub apply_user_id: Uuid,
}
