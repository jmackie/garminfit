use byteorder::{
    LittleEndian,
    ReadBytesExt,
};
use dyncrc16::{
    CRC16,
    CRC_SIZE,
};
use error::{
    Error,
    Result,
};
use std::{
    collections::HashMap,
    io::{
        Seek,
        SeekFrom,
    },
};
use types::record::{
    self,
    Definition,
    Record,
};

pub struct File {
    pub header:  Header,
    pub records: Vec<Record>,
}

impl File {
    pub fn decode<R: Seek + ReadBytesExt>(r: &mut R) -> Result<Self> {
        let mut _crc = CRC16::new(); // TODO

        let header =
            Header::decode(r).map_err(Error::decoding("file header"))?;

        // TODO: check crc here

        // Initialise loop variables
        let mut records = Vec::new(); // what we want from the loop.
        let mut local_mesgs: HashMap<u8, Definition> = HashMap::new();
        let mut bytes_left = header.data_size as u64;
        let mut count = 1;

        while bytes_left > 0 {
            let position_before = current_position(r)?;

            let record = Record::decode(r, &mut local_mesgs)
                .map_err(Error::decoding(format!("record #{}", count)))?;

            // If we got a definition message we need
            // to add it to the `local_mesgs` map
            if let record::Message::Definition(ref mesg) = record.content {
                // TODO: cloning here seems hacky...
                local_mesgs
                    .insert(record.header.local_mesg_num(), mesg.clone());
            }

            // TODO: check crc after every record
            records.push(record);

            let position_after = current_position(r)?;
            bytes_left -= position_after - position_before;
            count += 1;
        }

        Ok(File {
            header,
            records,
        })
    }
}

pub struct Header {
    size:             u8,
    protocol_version: u8,
    profile_version:  u16,
    data_size:        u32,
    data_type:        [u8; 4],
    crc:              Option<u16>,
}

const SIZE_NO_CRC: u8 = 12;
const SIZE_HAS_CRC: u8 = SIZE_NO_CRC + CRC_SIZE;
const MAX_SUPPORTED_PROTOCOL: u8 = 0x20; // v20.0.0

impl Header {
    pub(crate) fn decode<R: ReadBytesExt>(r: &mut R) -> Result<Self> {
        let size = r.read_u8().map_err(Error::reading("size"))?;

        // Check that the header size is valid
        match size {
            SIZE_HAS_CRC | SIZE_NO_CRC => (),
            _ => return Err(Error::unknown_file_header_size(size)),
        }

        let protocol_version =
            r.read_u8().map_err(Error::reading("protocol version"))?;

        // Check that we support this protocol version
        if protocol_version.major() > MAX_SUPPORTED_PROTOCOL.major() {
            return Err(Error::unsupported_protocol_version(
                MAX_SUPPORTED_PROTOCOL.major(),
                protocol_version.major(),
            ))
        }

        let profile_version = r
            .read_u16::<LittleEndian>()
            .map_err(Error::reading("profile version"))?;

        let data_size = r
            .read_u32::<LittleEndian>()
            .map_err(Error::reading("data size"))?;

        // Check magic number
        let mut data_type = [0; 4];
        r.read(&mut data_type).map_err(Error::reading("data type"))?;
        if &data_type != b".FIT" {
            return Err(Error::not_fit())
        }

        // fcrc.write(&[size as u8])
        //    .expect("dyncrc16 write implementation can't fail");
        // fcrc.write(&buf[..size as usize])
        //    .expect("dyncrc16 write implementation can't fail");

        let crc = match size {
            SIZE_NO_CRC => None,
            SIZE_HAS_CRC => {
                Some(
                    r.read_u16::<LittleEndian>()
                        .map_err(Error::reading("crc"))?,
                )
            },
            _ => panic!("invalid size should have been caught earlier"),
        };

        Ok(Header {
            size,
            protocol_version,
            profile_version,
            data_size,
            data_type,
            crc,
        })
    }
}

/// Small utility trait for working with version bytes
/// above.
trait Version {
    fn major(&self) -> u8;
    fn minor(&self) -> u8;
}

impl Version for u8 {
    fn major(&self) -> u8 {
        const MAJOR_SHIFT: u8 = 4;
        const MAJOR_MASK: u8 = 0x0F << MAJOR_SHIFT;

        (self & MAJOR_MASK) >> MAJOR_SHIFT
    }

    fn minor(&self) -> u8 {
        const MINOR_MASK: u8 = 0x0f;

        self & MINOR_MASK
    }
}

fn current_position<R: Seek>(r: &mut R) -> Result<u64> {
    r.seek(SeekFrom::Current(0)).map_err(Error::seek)
}
