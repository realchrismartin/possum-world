use web_sys::WebSocket;
use web_sys::MessageEvent;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::js_sys::{ArrayBuffer,Uint8Array};
use web_sys::BinaryType;
use std::sync::Arc;
use std::sync::Mutex;
use std::convert::TryInto;
use crate::util::logging::log;

static RATE_LIMIT : f32 = 50.0;

pub struct OutboundMessage
{
    x: f32,
    y: f32
}

impl OutboundMessage
{
    pub fn new(x: f32, y: f32) -> Self
    {
        Self
        {
            x,
            y
        }
    }

    pub fn as_vec_u8(&self) -> Vec<u8>
    {
        let mut pos_vec = Vec::<u8>::new();

        let x_bytes = self.x.to_le_bytes();
        let y_bytes = self.y.to_le_bytes();

        pos_vec.extend_from_slice(&x_bytes);
        pos_vec.extend_from_slice(&y_bytes);

        pos_vec
    }
}

pub enum MessageType
{
    Update,
    Departure
}

pub struct InboundMessage
{
    uuid: String,
    x: f32,
    y: f32,
    message_type: MessageType
}

impl InboundMessage
{
    pub fn from_update_message_bytes(bytes: &[u8;44]) -> Self
    {
        let mut x = 0.0;
        let mut y = 0.0;

        match &bytes[36..40].try_into()
        {
            Ok(x_s) => {
                x = f32::from_le_bytes(*x_s);
            }
            Err(_) => {}
        };

        match &bytes[40..44].try_into()
        {
            Ok(y_s) => {
                y = f32::from_le_bytes(*y_s);
            }
            Err(_) => {}
        };

        Self
        {
            uuid: String::from_utf8_lossy(&bytes[0..36]).to_string(),
            x: x,
            y: y,
            message_type: MessageType::Update
        }
    }

    pub fn from_departure_message_bytes(bytes: &[u8;36]) -> Self
    {
        Self
        {
            uuid: String::from_utf8_lossy(&bytes[0..36]).to_string(),
            x: 0.0,
            y: 0.0, 
            message_type: MessageType::Departure
        }
    }

    pub fn x(&self) -> &f32
    {
        &&self.x
    }

    pub fn y(&self) -> &f32
    {
        &&self.y
    }

    pub fn uuid(&self) -> &String
    {
        &&self.uuid
    }

    pub fn message_type(&self) -> &MessageType
    {
        &&self.message_type
    }
}

pub struct ServerConnection
{
    socket: Option<WebSocket>,
    time_since_last_update: f32,
    inbound_message_queue: Arc<Mutex<Vec<InboundMessage>>>
}

impl ServerConnection
{
    pub fn new() -> Self
    {
        let inbound_message_queue = Arc::new(Mutex::new(Vec::<InboundMessage>::new()));

        let socket = match WebSocket::new("ws://127.0.0.1:8000")
        {
            Ok(ws) => {

                let queue_ref = inbound_message_queue.clone();

                ws.set_binary_type(BinaryType::Arraybuffer);
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    if let Ok(abuf) = e.data().dyn_into::<ArrayBuffer>() {
                        let byte_array = Uint8Array::new(&abuf);

                        if byte_array.byte_length() == 44
                        {
                            let mut raw_bytes : [u8;44] = [0;44];
                            byte_array.copy_to(&mut raw_bytes);
                            queue_ref.lock().unwrap().push(InboundMessage::from_update_message_bytes(&raw_bytes)); 
                        } else if byte_array.byte_length() == 36 
                        {
                            let mut raw_bytes : [u8;36] = [0;36];
                            byte_array.copy_to(&mut raw_bytes);
                            queue_ref.lock().unwrap().push(InboundMessage::from_departure_message_bytes(&raw_bytes)); 
                        } else
                        {
                            log(&format!("Received a message with the wrong number of bytes, got {}", byte_array.byte_length()));
                        }
                    }
                });
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget();
                Some(ws)
            }
            Err(_) => 
            {
                //TODO: log
                None
            }
        };

        Self
        {
            socket,
            inbound_message_queue: inbound_message_queue,
            time_since_last_update: RATE_LIMIT
        }
    }

    pub fn send_message_if_ready(&mut self, message: &OutboundMessage, delta_time: f32)
    {
        self.time_since_last_update += delta_time;

        if self.time_since_last_update < RATE_LIMIT
        {
            return;
        }

        self.time_since_last_update = 0.0;

        if self.socket.is_none()
        {
            return;
        }

        let socket = self.socket.as_ref().unwrap();

        if socket.ready_state() != 1
        {
            return;
        }

        let message_byte_array = message.as_vec_u8();

        let buffer = ArrayBuffer::new(message_byte_array.len() as u32);
        let byte_array = Uint8Array::new(&buffer);

        for i in 0..message_byte_array.len()
        {
            //TODO: make this better
            //Remove/make casts safer, don't iterate
            byte_array.fill(message_byte_array[i],i as u32,(i+1) as u32); 
        }

        match socket.send_with_array_buffer(&buffer)
        {
            Ok(_) => {},
            Err(err) => log(format!("error sending message: {:?}", err).as_str()),
        }
    }

    pub fn receive_inbound_messages(&mut self, functor: &mut dyn FnMut(&InboundMessage))
    {
        let mut locked_queue = self.inbound_message_queue.lock().unwrap();

        for i in locked_queue.iter()
        {
            functor(i);
        }

        locked_queue.clear();
    }
}