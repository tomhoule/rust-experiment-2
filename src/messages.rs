use serde_json as json;
use uuid::Uuid;
use std::iter::{FromIterator, IntoIterator};

// #[derive(Debug)]
// enum ErrorCode {
//     ParseError, // -32700
//     InvalidRequest, // -32600
//     MethodNotFound, // -32601
//     InvalidParams, // -32602
//     InternalError, // -32603
//     serverErrorStart, // -32099
//     serverErrorEnd, // -32000
// }


#[derive(Debug)]
pub enum IncomingMessage {
    Response(ResponseMessage),
    Notification(ServerNotification),
    MultipleMessages(Vec<IncomingMessage>),
}

#[derive(Debug)]
pub enum OutgoingMessage {
    Request(RequestMessage),
    Notification(Notification),
}

impl FromIterator<IncomingMessage> for IncomingMessage {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = IncomingMessage>
    {
        IncomingMessage::MultipleMessages(Vec::from_iter(iter))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestMessage {
    pub jsonrpc: String,
    pub id: Uuid,
    pub method: String,
    pub params: json::Value,
}

impl RequestMessage {
    pub fn new(method: String, params: json::Value) -> Self {
        RequestMessage {
            jsonrpc: "2.0".to_string(),
            id: Uuid::new_v4(),
            method: method,
            params: params,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ResponseMessage {
    pub jsonrpc: String,
    pub id: Uuid,
    pub result: Option<json::Value>,
    pub error: Option<json::Value>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Notification {
    pub jsonrpc: String,
    pub method: String,
    pub params: json::Value,
}

impl Notification {
    pub fn new(method: String, params: json::Value) -> Self {
        Notification {
            jsonrpc: "2.0".to_string(),
            method: method,
            params: params,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ServerNotification {
    Other(Notification)
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ResponseError<T> {
    code: i32,
    message: String,
    data: T,
}
