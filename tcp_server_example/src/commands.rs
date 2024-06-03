use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    GetHelloWorld,
    PostGreeting { message: String },
}
