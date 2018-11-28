use bits::Bits;
use byteorder::{
    BigEndian,
    ByteOrder,
    LittleEndian,
    ReadBytesExt,
};
use error::{
    Error,
    Result,
};
use profile;
use std::{
    collections::HashMap,
    convert::TryFrom,
};

pub struct Record {
    pub header:  Header,
    pub content: Message,
}

impl Record {
    pub(crate) fn decode<R: ReadBytesExt>(
        r: &mut R,
        local_mesgs: &HashMap<u8, Definition>,
    ) -> Result<Self> {
        let header = Header::decode(r).map_err(Error::decoding("header"))?;

        // Decode record content according to the header we got
        let content = match header {
            Header::Definition {
                ..
            } => {
                Message::Definition(
                    Definition::decode(r)
                        .map_err(Error::decoding("definition message"))?,
                )
            },

            Header::Data {
                local_mesg_num,
            } => {
                // Lookup the definition the defines the upcoming `Data`
                // message
                let definition = local_mesgs
                    .get(&local_mesg_num)
                    .ok_or(Error::missing_definition(local_mesg_num))?;

                match definition.arch {
                    Architecture::LittleEndian => {
                        Message::Data(
                            Data::decode::<R, LittleEndian>(r, definition)
                                .map_err(Error::decoding("data message"))?,
                        )
                    },
                    Architecture::BigEndian => {
                        Message::Data(
                            Data::decode::<R, BigEndian>(r, definition)
                                .map_err(Error::decoding("data message"))?,
                        )
                    },
                }
            },

            Header::CompressedTimestamp {
                ..
            } => Message::CompressedTimestamp,
        };

        Ok(Record {
            header,
            content,
        })
    }
}

pub enum Header {
    Definition {
        local_mesg_num: u8,
    },
    Data {
        local_mesg_num: u8,
    },
    CompressedTimestamp {
        local_mesg_num: u8,
        time_offset:    u8, // seconds
    },
}

impl Header {
    fn decode<R: ReadBytesExt>(r: &mut R) -> Result<Self> {
        let byte = r.read_u8().map_err(Error::reading("byte"))?;

        // "A value of 0 in Bit 7 of the record header indicates
        // that this is a Normal Header."
        if byte.bit_not_set(7) {
            // Normal Message Type (bit 6)
            // 1: Definition Message
            // 0: Data Message
            if byte.bit_is_set(6) {
                Ok(Header::Definition {
                    local_mesg_num: byte.bit_range(0, 3)
                })
            }
            else {
                Ok(Header::Data {
                    local_mesg_num: byte.bit_range(0, 3)
                })
            }
        }
        else {
            Ok(Header::CompressedTimestamp {
                local_mesg_num: byte.bit_range(5, 6),
                time_offset:    byte.bit_range(0, 4),
            })
        }
    }

    /// Convenience method to access the `local_mesg_num`
    /// field common to all `Header` types.
    pub fn local_mesg_num(&self) -> u8 {
        match self {
            Header::Definition {
                local_mesg_num,
            } => *local_mesg_num,
            Header::Data {
                local_mesg_num,
            } => *local_mesg_num,
            Header::CompressedTimestamp {
                local_mesg_num, ..
            } => *local_mesg_num,
        }
    }
}

pub enum Message {
    Definition(Definition),
    Data(Data),
    CompressedTimestamp, // TODO (CompressedTimestamp),
}

#[derive(Debug, Clone)]
pub struct Definition {
    // NOTE: Reserved byte here!
    arch:            Architecture,
    global_mesg_num: u16,
    nfields:         u8,
    field_defs:      Vec<FieldDefinition>,
}

impl Definition {
    pub(super) fn decode<R: ReadBytesExt>(r: &mut R) -> Result<Self> {
        // NOTE: Discarding the reserved byte
        r.read_u8().map_err(Error::reading("reserved byte"))?;

        // TODO: It would be nice to check this architecture value
        // and get back the corresponding reader in one hit
        let arch = r
            .read_u8()
            .map_err(Error::reading("architecture byte"))
            .and_then(Architecture::try_from)?;

        let global_mesg_num = match arch {
            Architecture::LittleEndian => {
                r.read_u16::<LittleEndian>()
                    .map_err(Error::reading("global message number"))?
            },
            Architecture::BigEndian => {
                r.read_u16::<BigEndian>()
                    .map_err(Error::reading("global message number"))?
            },
        };

        let nfields =
            r.read_u8().map_err(Error::reading("number of fields"))?;

        let mut field_defs = Vec::with_capacity(nfields as usize);
        for i in 0..nfields {
            let field_def = FieldDefinition::decode(r)
                .map_err(Error::reading(format!("field definition #{}", i)))?;
            field_defs.push(field_def);
        }

        Ok(Definition {
            arch,
            global_mesg_num,
            nfields,
            field_defs,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FieldDefinition {
    num:            u8,
    size:           u8,
    _base_type_num: u8,
}

impl FieldDefinition {
    pub(super) fn decode<R: ReadBytesExt>(r: &mut R) -> Result<Self> {
        let num = r.read_u8().map_err(Error::reading("number"))?;

        let size = r.read_u8().map_err(Error::reading("size"))?;

        let _base_type_num =
            r.read_u8().map_err(Error::reading("base type"))?;

        Ok(FieldDefinition {
            num,
            size,
            _base_type_num,
        })
    }
}

pub struct Data(pub Vec<profile::messages::Message>);

// TODO
impl Data {
    pub(super) fn decode<R: ReadBytesExt, T: ByteOrder>(
        r: &mut R,
        definition: &Definition,
    ) -> Result<Self> {
        let mut mesgs = Vec::with_capacity(definition.field_defs.len());

        for field_def in definition.field_defs.iter() {
            let mut buffer = vec![0; field_def.size as usize];
            r.read(&mut buffer).map_err(Error::reading("buffer"))?;
            let mesg = profile::messages::Message::decode::<T>(
                &buffer,
                definition.global_mesg_num,
                field_def.num,
            )?;
            mesgs.push(mesg);
        }
        Ok(Data(mesgs))
    }
}

#[derive(Debug, Clone)]
enum Architecture {
    LittleEndian = 0,
    BigEndian = 1,
}

impl TryFrom<u8> for Architecture {
    type Error = Error;

    fn try_from(n: u8) -> Result<Architecture> {
        match n {
            0 => Ok(Architecture::LittleEndian),
            1 => Ok(Architecture::BigEndian),
            _ => Err(Error::unknown_architecture(n)),
        }
    }
}
