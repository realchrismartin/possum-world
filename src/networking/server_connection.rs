use web_sys::WebSocket;
use web_sys::MessageEvent;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::js_sys::{ArrayBuffer,Uint8Array};
use web_sys::BinaryType;
use crate::util::logging::log;

static RATE_LIMIT : f32 = 2000.0;

pub struct Message
{
    x: f32,
    y: f32
}

impl Message
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

pub struct ServerConnection
{
    socket: Option<WebSocket>,
    time_since_last_update: f32
}

impl ServerConnection
{
    pub fn new() -> Self
    {
        let socket = match WebSocket::new("ws://127.0.0.1:8000")
        {
            Ok(ws) => {
                ws.set_binary_type(BinaryType::Arraybuffer);
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    if let Ok(abuf) = e.data().dyn_into::<ArrayBuffer>() {
                        /*
                        for i in Uint8Array::new(&abuf).to_vec().into_iter()
                        {
                            log(&format!("Received a byte from the server: {}",i));
                        }
                        */
                    }
                });
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget();
                Some(ws)
            }
            Err(e) => 
            {
                //TODO: log
                None
            }
        };


        Self
        {
            socket,
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
}