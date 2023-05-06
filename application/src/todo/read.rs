use serde::Deserialize;


#[derive(Deserialize)]
pub struct ReadTodoRequest {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "completed")]
    pub completed: bool,
}

pub fn read() {
}
