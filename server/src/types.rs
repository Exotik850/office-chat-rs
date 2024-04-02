use rocket::{request::{FromRequest, Outcome}, Request};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone)]
pub struct UserID(pub(crate) String);

#[async_trait]
impl<'r> FromRequest<'r> for UserID {
  type Error = &'static str;
  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let cookie_jar = req.cookies();
    let user_id = cookie_jar.get_private("user_id");
    match user_id {
      Some(id) => Outcome::Success(UserID(id.value().to_string())),
      None => Outcome::Error((rocket::http::Status::Unauthorized, "No user_id cookie found")),
    }
  }
}



#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub struct MessageID(pub(crate) usize);

#[derive(Serialize, Deserialize, PartialEq, Debug, Eq, Hash, Clone)]
pub struct ChatRoomID(pub(crate) String);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct User {
    pub name: UserID,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ChatMessage {
    pub room: ChatRoomID,
    pub sender: UserID,
    pub content: String,
    pub timestamp: u64,
}

impl Into<Vec<u8>> for ChatMessage {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

impl std::fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ChatRoom {
    pub name: ChatRoomID,
    pub participants: Vec<UserID>,
    pub content: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UserData {
  pub name: UserID,
  pub rooms: Vec<ChatRoomID>,
  pub active: Option<ChatRoomID>,
}