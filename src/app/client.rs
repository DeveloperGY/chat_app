use serde::Deserialize;

#[derive(Clone, Copy, Deserialize)]
pub struct Client {
    pub id: usize
}