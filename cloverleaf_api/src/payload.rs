use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum PayloadType {
    Offer,
    Answer,
    Candidate,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Payload {
    pub pt: PayloadType,
    pub payload: String,
}