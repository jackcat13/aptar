use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DiscordTokenParams {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub code: String,
    pub redirect_url: String,
}
