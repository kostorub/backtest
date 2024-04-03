use serde::Serialize;

#[derive(Debug, Serialize)]
struct Error {
    msg: String,
    status: u16,
}
