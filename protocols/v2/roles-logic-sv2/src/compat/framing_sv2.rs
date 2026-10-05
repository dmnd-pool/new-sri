//! Compatibility for the former generic frame, backed by the current SRI framing codec.

pub use ::framing_sv2::{error, Error, SV2_FRAME_CHUNK_SIZE, SV2_FRAME_HEADER_SIZE};

pub mod header {
    use super::Error;
    use ::binary_sv2::{
        decodable::{DecodableField, FieldMarker},
        encodable::EncodableField,
        Deserialize, GetSize,
    };

    /// The legacy header methods over SRI's current header representation.
    #[derive(Debug, Clone, Copy)]
    pub struct Header(::framing_sv2::header::Header);

    impl Header {
        pub const SIZE: usize = ::framing_sv2::header::Header::SIZE;

        pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
            ::framing_sv2::header::Header::from_bytes(bytes).map(Self)
        }

        pub fn msg_type(&self) -> u8 {
            self.0.msg_type()
        }
        pub fn ext_type(&self) -> u16 {
            self.0.ext_type()
        }
        pub fn channel_msg(&self) -> bool {
            self.0.channel_msg()
        }

        /// The encrypted payload length, including the Poly1305 tag on each chunk.
        pub fn encrypted_len(&self) -> usize {
            const MAC_LEN: usize = 16;
            let len = self.0.payload_length();
            let chunk_size = super::SV2_FRAME_CHUNK_SIZE - MAC_LEN;
            let chunks = len / chunk_size + usize::from(len % chunk_size != 0);
            len + chunks * MAC_LEN
        }
    }

    impl core::ops::Deref for Header {
        type Target = ::framing_sv2::header::Header;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl From<::framing_sv2::header::Header> for Header {
        fn from(header: ::framing_sv2::header::Header) -> Self {
            Self(header)
        }
    }

    impl From<Header> for ::framing_sv2::header::Header {
        fn from(header: Header) -> Self {
            header.0
        }
    }

    impl GetSize for Header {
        fn get_size(&self) -> usize {
            Self::SIZE
        }
    }

    impl<'a> From<Header> for EncodableField<'a> {
        fn from(header: Header) -> Self {
            header.0.into()
        }
    }

    impl<'a> Deserialize<'a> for Header {
        fn get_structure(data: &[u8]) -> Result<Vec<FieldMarker>, ::binary_sv2::Error> {
            <::framing_sv2::header::Header as Deserialize<'a>>::get_structure(data)
        }

        fn from_decoded_fields(
            fields: Vec<DecodableField<'a>>,
        ) -> Result<Self, ::binary_sv2::Error> {
            <::framing_sv2::header::Header as Deserialize<'a>>::from_decoded_fields(fields)
                .map(Self)
        }
    }
}

pub mod framing {
    pub use ::framing_sv2::framing::{
        EncodableFrame, HandshakeMessage, MessageFrame, SerializedFrame, SizeHint,
    };

    use super::{header::Header, Error};
    use ::binary_sv2::{GetSize, Serialize};

    #[derive(Clone, Debug)]
    enum Payload<T, B> {
        Message(T),
        Serialized(SerializedFrame<B>),
    }

    /// The legacy frame API. This frame can also be passed to the current SRI encoder.
    #[derive(Clone, Debug)]
    pub struct Sv2Frame<T, B> {
        header: Header,
        payload: Payload<T, B>,
    }

    impl<T: Serialize + GetSize, B: AsMut<[u8]> + AsRef<[u8]>> Sv2Frame<T, B> {
        /// Serialize into a buffer that has room for this frame.
        ///
        /// As in the legacy API, any bytes following the frame are left untouched.
        pub fn serialize(self, dst: &mut [u8]) -> Result<(), Error> {
            let expected = self.encoded_length();
            if dst.len() < expected {
                return Err(Error::UnexpectedDestinationLength {
                    expected,
                    actual: dst.len(),
                });
            }
            self.encode_into(&mut dst[..expected])
        }

        /// Return the payload of a received, serialized frame.
        pub fn payload(&mut self) -> &mut [u8] {
            match &mut self.payload {
                Payload::Serialized(frame) => frame.payload(),
                Payload::Message(_) => panic!("Sv2Frame is not yet serialized."),
            }
        }

        pub fn get_header(&self) -> Option<Header> {
            Some(self.header)
        }

        pub fn from_bytes(bytes: B) -> Result<Self, isize> {
            SerializedFrame::from_bytes(bytes)
                .map(Self::from)
                .map_err(legacy_size_hint)
        }

        pub fn from_bytes_unchecked(bytes: B) -> Self {
            let header = Header::from_bytes(bytes.as_ref()).expect("Invalid header");
            Self::from(SerializedFrame::from_parts(header.into(), bytes))
        }

        /// Zero for a complete frame, a positive count for missing bytes, and a
        /// negative count for surplus bytes, as expected by legacy decoders.
        pub fn size_hint(bytes: &[u8]) -> isize {
            SerializedFrame::<B>::parse_header(bytes)
                .map(|_| 0)
                .unwrap_or_else(legacy_size_hint)
        }

        pub fn encoded_length(&self) -> usize {
            self.header.payload_length() + Header::SIZE
        }

        pub fn from_message(
            message: T,
            message_type: u8,
            extension_type: u16,
            channel_msg: bool,
        ) -> Option<Self> {
            let extension_type = (extension_type & 0x7fff) | (u16::from(channel_msg) << 15);
            let len = message.get_size();
            if len > 0x00ff_ffff {
                return None;
            }
            let mut bytes = [0; Header::SIZE];
            bytes[..2].copy_from_slice(&extension_type.to_le_bytes());
            bytes[2] = message_type;
            bytes[3..].copy_from_slice(&(len as u32).to_le_bytes()[..3]);
            let header = Header::from_bytes(&bytes).ok()?;
            Some(Self {
                header,
                payload: Payload::Message(message),
            })
        }
    }

    impl<T, B> Sv2Frame<T, B> {
        pub fn map<C>(self, fun: fn(T) -> C) -> Sv2Frame<C, B> {
            let payload = match self.payload {
                Payload::Message(message) => Payload::Message(fun(message)),
                Payload::Serialized(frame) => Payload::Serialized(frame),
            };
            Sv2Frame {
                header: self.header,
                payload,
            }
        }
    }

    impl<T, B: AsMut<[u8]> + AsRef<[u8]>> From<SerializedFrame<B>> for Sv2Frame<T, B> {
        fn from(frame: SerializedFrame<B>) -> Self {
            Self {
                header: frame.header().into(),
                payload: Payload::Serialized(frame),
            }
        }
    }

    impl<T: Serialize + GetSize, B: AsMut<[u8]> + AsRef<[u8]>> EncodableFrame for Sv2Frame<T, B> {
        fn encoded_length(&self) -> usize {
            Sv2Frame::encoded_length(self)
        }

        fn encode_into(self, dst: &mut [u8]) -> Result<(), Error> {
            match self.payload {
                Payload::Message(message) => MessageFrame::from_message(
                    message,
                    self.header.msg_type(),
                    self.header.ext_type(),
                    self.header.channel_msg(),
                )?
                .encode_into(dst),
                Payload::Serialized(frame) => frame.encode_into(dst),
            }
        }
    }

    fn legacy_size_hint(hint: SizeHint) -> isize {
        match hint {
            SizeHint::Missing(len) => len as isize,
            SizeHint::Surplus(len) => -(len as isize),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::header::Header;

    #[test]
    fn legacy_header_encrypted_length_matches_current_codec() {
        for len in [0u32, 1, 65_519, 65_520, 131_038, 131_039] {
            let mut bytes = [0u8; Header::SIZE];
            bytes[3..].copy_from_slice(&len.to_le_bytes()[..3]);
            let header = Header::from_bytes(&bytes).unwrap();
            assert_eq!(
                header.encrypted_len(),
                ::codec_sv2::encrypted_payload_length(&header),
            );
        }
    }
}
