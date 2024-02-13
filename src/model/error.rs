#[derive(Debug, Clone, strum_macros::AsRefStr, serde::Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // Model Errors
    ModelEmptyTitle,
    ModelTicketIdNotFound(u64),
}
