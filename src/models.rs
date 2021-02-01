use serde::Serialize;
#[derive(Serialize)]
pub struct Status {
    pub status: String
}
use serde::Deserialize;
#[derive(Deserialize)]
pub struct Row<'a> {
    pub iden: &'a str,
    pub name: &'a str,
    pub gender: &'a str,
    pub civile: &'a str,
    pub birth: &'a str,
    pub phone: &'a str,
    pub dirr: &'a str,
    pub mail: &'a str,
}
