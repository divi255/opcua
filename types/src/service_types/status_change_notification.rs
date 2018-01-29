// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use status_codes::StatusCode;
use basic_types::DiagnosticInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct StatusChangeNotification {
    pub status: StatusCode,
    pub diagnostic_info: DiagnosticInfo,
}

impl BinaryEncoder<StatusChangeNotification> for StatusChangeNotification {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status.byte_len();
        size += self.diagnostic_info.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status.encode(stream)?;
        size += self.diagnostic_info.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status = StatusCode::decode(stream)?;
        let diagnostic_info = DiagnosticInfo::decode(stream)?;
        Ok(StatusChangeNotification {
            status,
            diagnostic_info,
        })
    }
}