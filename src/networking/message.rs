use std::convert::TryInto;
use crate::util::logging::log;

pub enum MessageType
{
    Update,
    Departure,
    ChatMessage,
    Unknown
}

impl MessageType
{
    pub fn id(&self) -> i32
    {
        match self {
            MessageType::Unknown => 0, //Or any other code
            MessageType::Update => 1,
            MessageType::Departure => 2,
            MessageType::ChatMessage => 3
        }
    }

    pub fn from_id(id: i32) -> MessageType
    {
        match id {
            1 => MessageType::Update,
            2 => MessageType::Departure,
            3 => MessageType::ChatMessage,
            _ => MessageType::Unknown,
        }
    }
}

pub struct Message
{
    message_type: MessageType,
    uuid: Option<String>,
    x: Option<f32>,
    y: Option<f32>,
    chat_message: Option<String>
}

impl Message
{
    pub fn new_update_message(x: f32, y: f32) -> Self
    {
        Self
        {
            message_type: MessageType::Update,
            uuid: None,
            x: Some(x),
            y: Some(y),
            chat_message: None
        }
    }

    pub fn new_departure_message() -> Self
    {
        Self
        {
            message_type: MessageType::Departure,
            uuid: None,
            x: None,
            y: None,
            chat_message: None
        }
    }

    pub fn new_chat_message(chat_message: String) -> Self
    {
        Self
        {
            message_type: MessageType::ChatMessage,
            uuid: None,
            x: None,
            y: None,
            chat_message: Some(chat_message)
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Option<Self>
    {
        let message = match Self::from_message_type_bytes(&bytes)
        {
            MessageType::Update => Self::from_update_message_bytes(&bytes),
            MessageType::Departure => Self::from_departure_message_bytes(&bytes),
            MessageType::ChatMessage => Self::from_chat_message_bytes(&bytes),
            MessageType::Unknown => None
        };

        message
    }

    pub fn to_bytes(&self) -> Vec<u8>
    {
        let bytes = match self.message_type
        {
            MessageType::Update => self.to_update_message_bytes(),
            MessageType::Departure => self.to_departure_message_bytes(),
            MessageType::ChatMessage => self.to_chat_message_bytes(),
            MessageType::Unknown => Vec::<u8>::new()
        };
        
        bytes
    }

    fn from_message_type_bytes(bytes: &Vec<u8>) -> MessageType
    {
        if bytes.len() < 4
        {
            return MessageType::Unknown;
        }

        let message_type = match &bytes[0..4].try_into()
        {
            Ok(b) => {
                MessageType::from_id(i32::from_le_bytes(*b))
            }
            Err(_) => {
                MessageType::Unknown
            }
        };

        message_type
    }

    fn to_update_message_bytes(&self) -> Vec<u8>
    {
        //Outgoing Update: 4 (type) + 8 (x,y)
        let mut bytes = Vec::<u8>::new();

        bytes.extend_from_slice(&self.message_type.id().to_le_bytes()); //4

        if self.x.is_none() || self.y.is_none()
        {
            return bytes;
        }

        bytes.extend_from_slice(&self.x.unwrap().to_le_bytes()); //4
        bytes.extend_from_slice(&self.y.unwrap().to_le_bytes()); //4

        bytes
    }

    fn from_update_message_bytes(bytes: &Vec<u8>) -> Option<Self>
    {
        //Incoming Update: 4 (type) + 36 (uuid) + 8 (x, y)

        if bytes.len() < 48
        {
            return None;
        }

        let x = match &bytes[40..44].try_into()
        {
            Ok(b) => f32::from_le_bytes(*b),
            Err(_) => 0.0
        };

        let y = match &bytes[44..48].try_into()
        {
            Ok(b) => f32::from_le_bytes(*b),
            Err(_) => 0.0
        };

        Some(Self
        {
            message_type: MessageType::Update,
            uuid: Some(String::from_utf8_lossy(&bytes[4..40]).to_string()),
            x: Some(x),
            y: Some(y),
            chat_message: None
        })
    }

    fn to_departure_message_bytes(&self) -> Vec<u8>
    {
        //NB This will never actually be sent - server generates these
        //Outgoing Departure: 4 (type)
        let mut bytes = Vec::<u8>::new();
        bytes.extend_from_slice(&self.message_type.id().to_le_bytes()); //4
        bytes
    }

    fn from_departure_message_bytes(bytes: &Vec<u8>) -> Option<Self>
    {
        //Incoming Departure: 4 (type) + 36 (uuid)
        if bytes.len() < 40
        {
            return None;
        }

        Some(Self
        {
            message_type: MessageType::Departure,
            uuid: Some(String::from_utf8_lossy(&bytes[4..40]).to_string()),
            x: None,
            y: None,
            chat_message: None
        })
    }

    fn to_chat_message_bytes(&self) -> Vec<u8>
    {
        //Outgoing Chat: 4 (type) + n (message)
        let mut bytes = Vec::<u8>::new();
        bytes.extend_from_slice(&self.message_type.id().to_le_bytes()); //4

        if self.chat_message.is_none()
        {
            return bytes;
        }

        bytes.extend_from_slice(&self.chat_message.as_ref().unwrap().as_bytes());
        bytes
    }

    fn from_chat_message_bytes(bytes: &Vec<u8>) -> Option<Self>
    {
        //Incoming Chat: 4 (type) + 36 (uuid) + n (message)
        if bytes.len() < 41
        {
            return None;
        }

        Some(Self
        {
            message_type: MessageType::ChatMessage,
            uuid: Some(String::from_utf8_lossy(&bytes[4..40]).to_string()),
            x: None,
            y: None,
            chat_message: Some(String::from_utf8_lossy(&bytes[40..]).to_string())
        })
    }

    pub fn message_type(&self) -> &MessageType
    {
        &&self.message_type
    }

    pub fn uuid(&self) -> &Option<String>
    {
        &&self.uuid
    }

    pub fn x(&self) -> &Option<f32>
    {
        &&self.x
    }

    pub fn y(&self) -> &Option<f32>
    {
        &&self.y
    }

    pub fn chat_message(&self) -> &Option<String>
    {
        &&self.chat_message
    }
}
