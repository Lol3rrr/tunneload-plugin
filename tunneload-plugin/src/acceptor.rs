//! The General Acceptor-Interface that should be used to handle the
//! Communication between the Plugin and Tunneload regarding the Acceptor
//! related Connections and Information.

use std::convert::TryInto;

use crate::raw;

/// Registers the new Connection in Tunneload and creates the right
/// Receiver for the new Connection to then send Data to Tunneload
pub fn new_connection(id: i32) -> Receiver {
    unsafe {
        raw::acceptor_new_con(id);
    }
    Receiver::new(id)
}

/// A single Message that indicates Data that should be sent to a Connection,
/// established by this Plugin.
#[derive(Debug)]
pub struct Message {
    /// The ID of the connection to which this Message belongs
    pub id: i32,
    /// The actual underlying Data that should be send to the Connection
    pub data: Vec<u8>,
}

/// The Sender is responsible for receiving all the Messages that should
/// be send to the given Connection established by the Plugin
pub struct Sender;

impl Sender {
    /// Creates a new Sender
    ///
    /// ## Note:
    /// All the Sender-Instances share the same State so you only really
    /// need a single Instance as anything else is considered undefined
    /// behaviour
    pub fn new() -> Self {
        Self
    }

    /// Gets any pending Message from Tunneload
    pub async fn send(&mut self) -> Option<Message> {
        loop {
            let send_size = unsafe { raw::acceptor_has_send() };
            if send_size < 0 {
                tokio::task::yield_now().await;
                continue;
            }
            let size = send_size as usize;

            let mut buffer: Vec<u8> = Vec::with_capacity(size);
            unsafe {
                let worked = raw::acceptor_send(buffer.as_ptr() as i32) != 0;
                if !worked {
                    tokio::task::yield_now().await;
                    continue;
                }
                buffer.set_len(size);
            }

            let raw_id = &buffer[..4];
            let id = i32::from_be_bytes(raw_id.try_into().unwrap());

            buffer.copy_within(4.., 0);
            buffer.resize(buffer.len() - 4, 0);

            return Some(Message { id, data: buffer });
        }
    }
}

/// The Receiver-Half trough which you forward the received Data for
/// a given Connection to Tunneload
pub struct Receiver {
    id: i32,
}

impl Receiver {
    /// Creates a new Receiver with the given ID for the Connection
    fn new(id: i32) -> Self {
        Self { id }
    }

    /// This passes the given Data on to Tunneload to present it as
    /// just received Data from the Connection
    pub fn recv(&self, data: &[u8]) {
        let target = data.as_ptr();
        let size = data.len();

        unsafe {
            raw::acceptor_recv(self.id, target as i32, size as i32);
        }
    }
}
