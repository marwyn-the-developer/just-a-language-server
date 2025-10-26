use bytes::{Bytes, BytesMut};
use serde::Serialize;
use serde_json::Error;
use std::result::Result;
use tokio_util::codec::Decoder;

pub fn frame_message<T: Serialize>(obj: T) -> Result<Bytes, Error> {
    let j = serde_json::to_vec(&obj)?;
    let header = format!("Content-Length: {}\r\n\r\n", j.len());

    let mut buf = BytesMut::with_capacity(header.len() + j.len());
    buf.extend_from_slice(header.as_bytes());
    buf.extend_from_slice(&j);
    Ok(buf.freeze())
}

pub fn unframe_message(msg: Bytes) -> Result<Bytes, Box<dyn std::error::Error>> {
    let pos = msg
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or("Invalid message")?;

    let header = &msg.slice(..pos);
    let header_str = str::from_utf8(header)?;
    let size: usize = header_str
        .strip_prefix("Content-Length: ")
        .ok_or("Missing Content-Length header")?
        .parse()?;

    if msg.len() < pos + 4 + size {
        return Err("Incomplete message".into());
    }

    let content = msg.slice(pos + 4..pos + 4 + size);

    Ok(content)
}

pub struct LspDecoder;

impl Decoder for LspDecoder {
    type Item = Bytes;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let pos = match src.windows(4).position(|w| w == b"\r\n\r\n") {
            Some(p) => p,
            None => return Ok(None),
        };

        let header = str::from_utf8(&src[..pos])
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let size: usize = header
            .strip_prefix("Content-Length: ")
            .ok_or("Missing Content-Length header")
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
            .parse()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        if src.len() < pos + 4 + size {
            return Ok(None);
        }

        let total_length = pos + 4 + size;
        let whole_msg = src.split_to(total_length).freeze();

        Ok(Some(whole_msg))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    mod encoding_tests {
        use super::*;

        #[derive(Serialize)]
        struct EncodingExample {
            testing: bool,
        }

        #[test]
        fn test_encode() {
            let expected = String::from("Content-Length: 16\r\n\r\n{\"testing\":true}");
            let actual =
                frame_message(EncodingExample { testing: true }).expect("unexpected error");
            assert_eq!(actual, expected)
        }

        #[test]
        fn test_encode_error() {
            let mut map = HashMap::new();
            map.insert(vec![1, 2, 3], "value"); // should fail because of a non string key

            let result = frame_message(&map);

            assert!(result.is_err())
        }
    }

    mod decoding_tests {
        use super::*;
        #[test]
        fn test_decode() {
            let incoming_msg = b"Content-Length: 17\r\n\r\n{\"method\":\"test\"}";
            let msg =
                unframe_message(Bytes::copy_from_slice(incoming_msg)).expect("unexpected error");

            assert_eq!(msg, Bytes::from_static(b"{\"method\":\"test\"}"));
        }

        #[test]
        fn test_invalid_message() {
            let incoming_msg = b"Content-Length: 17{\"method\":\"test\"}";
            let result = unframe_message(Bytes::copy_from_slice(incoming_msg));
            assert!(result.is_err());

            let err = result.unwrap_err();
            assert_eq!(err.to_string(), "Invalid message");
        }

        #[test]
        fn test_missing_content_length() {
            let incoming_msg = b"\r\n\r\n{\"method\":\"test\"}";
            let result = unframe_message(Bytes::copy_from_slice(incoming_msg));
            assert!(result.is_err());

            let err = result.unwrap_err();
            assert_eq!(err.to_string(), "Missing Content-Length header");
        }
        #[test]
        fn test_invalid_content_length() {
            let incoming_msg = b"Content-Length: 1z7\r\n\r\n{\"method\":\"test\"}";
            let result = unframe_message(Bytes::copy_from_slice(incoming_msg));
            assert!(result.is_err());
        }
        #[test]
        fn test_incomplete_message() {
            let incoming_msg = b"Content-Length: 17\r\n\r\n{\"method\":\"test\"";
            let result = unframe_message(Bytes::copy_from_slice(incoming_msg));
            assert!(result.is_err());

            let err = result.unwrap_err();
            assert_eq!(err.to_string(), "Incomplete message");
        }
    }

    mod lsp_decoder_tests {
        use super::*;
        #[test]
        fn test_decode() {
            let mut incoming_msg =
                BytesMut::from("Content-Length: 17\r\n\r\n{\"method\":\"test\"}");
            let mut lsp_decoder = LspDecoder;
            let result = lsp_decoder.decode(&mut incoming_msg);
            assert!(result.is_ok());

            let content = result.unwrap().unwrap();
            assert_eq!(
                content,
                Bytes::from("Content-Length: 17\r\n\r\n{\"method\":\"test\"}")
            )
        }
        #[test]
        fn test_longer_message() {
            let mut incoming_msg =
                BytesMut::from("Content-Length: 17\r\n\r\n{\"method\":\"test\"} qwewqtz");
            let mut lsp_decoder = LspDecoder;
            let result = lsp_decoder.decode(&mut incoming_msg);
            assert!(result.is_ok());

            let content = result.unwrap().unwrap();
            assert_eq!(
                content,
                Bytes::from("Content-Length: 17\r\n\r\n{\"method\":\"test\"}")
            )
        }
    }
}
