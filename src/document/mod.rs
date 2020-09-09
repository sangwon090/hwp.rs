mod header;
mod doc_info;

use std::path::Path;
use std::io::Read;
use cfb;

use crate::error::InvalidDocumentError;

pub use header::HwpHeader;

const HWP_MAGIC: [u8; 32] = [0x48, 0x57, 0x50, 0x20, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

pub struct HwpDocument {
    header: HwpHeader,
}

impl HwpDocument {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<HwpDocument, InvalidDocumentError> {
        let mut file = cfb::open(path).unwrap();

        let header = {
            let mut stream = file.open_stream("/FileHeader").unwrap();
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap();
            HwpHeader::parse(&buffer).unwrap().1
        };

        let summary = {
            let mut stream = file.open_stream("/\x05HwpSummaryInformation").unwrap();
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap();
            buffer
        };

        let doc_info = {
            let mut stream = file.open_stream("/DocInfo").unwrap();
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap();
            buffer
        };

        let preview_text = {
            let mut stream = file.open_stream("/PrvText").unwrap();
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap().to_string()
        };

        let preview_image = {
            let mut stream = file.open_stream("/DocInfo").unwrap();
            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer).unwrap();
            buffer
        };

        if header.signature != HWP_MAGIC {
            return Err(InvalidDocumentError)
        }

        Ok(HwpDocument {
            header,
        })
    }
}