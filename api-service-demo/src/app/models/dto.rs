use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateShortLinkReq {
    pub url: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserResp {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteShortLinkReq {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteShortLinkResp {
    pub ok: bool,
}
