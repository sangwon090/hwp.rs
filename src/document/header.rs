use std::convert::TryInto;

pub struct HwpHeader {
    pub signature: [u8; 32],
    pub version: u32,
    pub properties: u32,
    pub license: u32,
    pub encrypt_version: u32,
    pub kogl: u8,
    pub reserved: [u8; 207],
}

impl HwpHeader {
    pub fn open(data: &[u8]) -> nom::IResult<&[u8], HwpHeader> {
        match nom::combinator::all_consuming(nom::sequence::tuple((
            nom::bytes::complete::take(32usize),
            nom::number::complete::be_u32,
            nom::number::complete::be_u32,
            nom::number::complete::be_u32,
            nom::number::complete::be_u32,
            nom::number::complete::be_u8,
            nom::bytes::complete::take(207usize),
        )))(data) {
            Ok((remaining_input, (
                signature,
                version,
                properties,
                license,
                encrypt_version,
                kogl,
                reserved_value,
            ))) => {
                let mut reserved: [u8; 207] = [0; 207];
                reserved.clone_from_slice(reserved_value);

                Ok((remaining_input, HwpHeader {
                    signature: signature.try_into().unwrap(),
                    version: version,
                    properties: properties,
                    license: license,
                    encrypt_version: encrypt_version,
                    kogl: kogl,
                    reserved: reserved,
                }))
            },
            Err(e) => Err(e),
        }
    }
}