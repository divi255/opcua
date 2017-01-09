use std::io::{Read, Write, Result};

use types::*;
use services::*;
use comms::*;

#[derive(Debug, PartialEq)]
pub enum SupportedMessage {
    Invalid(ObjectId),
    OpenSecureChannelRequest(OpenSecureChannelRequest),
    OpenSecureChannelResponse(OpenSecureChannelResponse),
    CloseSecureChannelRequest(CloseSecureChannelRequest),
    CloseSecureChannelResponse(CloseSecureChannelResponse),
}

impl BinaryEncoder<SupportedMessage> for SupportedMessage {
    fn byte_len(&self) -> usize {
        match *self {
            SupportedMessage::Invalid(object_id) => {
                panic!("Unsupported message {:?}", object_id);
            },
            SupportedMessage::OpenSecureChannelRequest(ref value) => value.byte_len(),
            SupportedMessage::OpenSecureChannelResponse(ref value) => value.byte_len(),
            SupportedMessage::CloseSecureChannelRequest(ref value) => value.byte_len(),
            SupportedMessage::CloseSecureChannelResponse(ref value) => value.byte_len(),
        }
    }

    fn encode(&self, stream: &mut Write) -> Result<usize> {
        match *self {
            SupportedMessage::Invalid(object_id) => {
                panic!("Unsupported message {:?}", object_id);
            },
            SupportedMessage::OpenSecureChannelRequest(ref value) => value.encode(stream),
            SupportedMessage::OpenSecureChannelResponse(ref value) => value.encode(stream),
            SupportedMessage::CloseSecureChannelRequest(ref value) => value.encode(stream),
            SupportedMessage::CloseSecureChannelResponse(ref value) => value.encode(stream),
        }
    }

    fn decode(stream: &mut Read) -> Result<SupportedMessage> {
        // THIS WILL NOT DO ANYTHING
        panic!("Cannot decode a stream to a supported message type");
    }
}

impl SupportedMessage {
    pub fn node_id(&self) -> NodeId {
        match *self {
            SupportedMessage::Invalid(object_id) => {
                panic!("Unsupported message {:?}", object_id);
            },
            SupportedMessage::OpenSecureChannelRequest(ref value) => value.node_id(),
            SupportedMessage::OpenSecureChannelResponse(ref value) => value.node_id(),
            SupportedMessage::CloseSecureChannelRequest(ref value) => value.node_id(),
            SupportedMessage::CloseSecureChannelResponse(ref value) => value.node_id(),
        }
    }

    pub fn chunk_message_type(&self) -> ChunkMessageType {
        match *self {
            SupportedMessage::Invalid(object_id) => {
                panic!("Unsupported message {:?}", object_id);
            },
            SupportedMessage::OpenSecureChannelRequest(_) => ChunkMessageType::OpenSecureChannel,
            SupportedMessage::OpenSecureChannelResponse(_) => ChunkMessageType::OpenSecureChannel,
            SupportedMessage::CloseSecureChannelRequest(_) => ChunkMessageType::CloseSecureChannel,
            SupportedMessage::CloseSecureChannelResponse(_) => ChunkMessageType::OpenSecureChannel,
        }
    }
}
