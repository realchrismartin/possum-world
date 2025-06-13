use web_sys::WebSocket;
use web_sys::MessageEvent;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::js_sys::{ArrayBuffer,Uint8Array};
use web_sys::BinaryType;
use std::sync::Arc;
use std::sync::Mutex;
use crate::networking::message::Message;
use crate::util::logging::log;

static RATE_LIMIT : f32 = 50.0;

pub struct ServerConnection
{
    socket: Option<WebSocket>,
    time_since_last_update: f32,
    inbound_message_queue: Arc<Mutex<Vec<Message>>>
}

impl ServerConnection
{
    pub fn new() -> Self
    {
        let inbound_message_queue = Arc::new(Mutex::new(Vec::<Message>::new()));

        let socket = match WebSocket::new("ws://127.0.0.1:8000")
        {
            Ok(ws) => {

                let queue_ref = inbound_message_queue.clone();

                ws.set_binary_type(BinaryType::Arraybuffer);
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    if let Ok(abuf) = e.data().dyn_into::<ArrayBuffer>() {
                        //TODO unnecessary copy?
                        match Message::from_bytes(Uint8Array::new(&abuf).to_vec())
                        {
                            Some(m) => {
                                queue_ref.lock().unwrap().push(m);
                            },
                            None => {
                                log(&format!("Failed to receive a message - not decoded properly"));
                            } 
                        };
                    } else
                    {
                        log(&format!("Failed convert inbound message data to arraybuffer"));
                    }
                });
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget();
                Some(ws)
            },
            Err(_) => 
            {
                log(&format!("Failed to open WebSocket connection"));
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

    pub fn send_message_if_ready(&mut self, message: &Message, delta_time: f32)
    {
        self.time_since_last_update += delta_time;

        if self.time_since_last_update < RATE_LIMIT
        {
            return;
        }

        self.time_since_last_update = 0.0;
        self.immediately_send_message(message);
    }

    pub fn immediately_send_message(&mut self, message: &Message)
    {
        if self.socket.is_none()
        {
            return;
        }

        let socket = self.socket.as_ref().unwrap();

        if socket.ready_state() != 1
        {
            return;
        }

        let message_byte_array = message.to_bytes();

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

    pub fn receive_inbound_messages(&mut self, functor: &mut dyn FnMut(&Message))
    {
        let mut locked_queue = self.inbound_message_queue.lock().unwrap();

        for i in locked_queue.iter()
        {
            functor(i);
        }

        locked_queue.clear();
    }
}