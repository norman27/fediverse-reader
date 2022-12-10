#[derive(serde::Deserialize)]
pub struct Account {
    pub avatar: String,
    pub url: String,
    pub username: String,
}

#[derive(serde::Deserialize)]
pub struct MediaAttachment {
    pub preview_url: String,
}

#[derive(serde::Deserialize)]
pub struct Status {
    pub account: Account,
    pub content: String,
    pub created_at: String,
    pub media_attachments: Vec<MediaAttachment>,
}