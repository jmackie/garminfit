//! Implements the Dynastream CRC-16 checksum.
use std::io;

// CRC16 represents the partial evaluation of a checksum.
pub(crate) struct CRC16(u16);

const CRC_TABLE: [u16; 16] = [
    0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401, 0xA001,
    0x6C00, 0x7800, 0xB401, 0x5000, 0x9C01, 0x8801, 0x4400,
];

pub const CRC_SIZE: u8 = 2;

impl CRC16 {
    /// Initialise a new CRC-16 checksum.
    pub(crate) fn new() -> CRC16 {
        CRC16(0)
    }

    // Hasher (?)
    pub(crate) fn sum_16(&self) -> u16 {
        self.0
    }

    /// Add data to the running checksum.
    fn update(&mut self, data: &[u8]) {
        for datum in data {
            self.update_byte(*datum)
        }
    }

    /// Add a datum to the running checksum.
    ///
    /// NOTE: This implementation is in the SDK docs.
    fn update_byte(&mut self, datum: u8) {
        // Compute checksum of lower four bits of byte.
        let tmp = CRC_TABLE[(self.0 & 0x0F) as usize];
        let d = (self.0 >> 4) & 0x0FFF;
        let d = d ^ tmp ^ CRC_TABLE[(datum & 0x0F) as usize];

        // Now compute checksum of upper four bits of byte.
        let tmp = CRC_TABLE[(d & 0x0F) as usize];
        let d = (d >> 4) & 0x0FFF;
        let d = d ^ tmp ^ CRC_TABLE[(datum >> 4 & 0x0F) as usize];

        self.0 = d
    }
}

impl io::Write for CRC16 {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    struct TestCase(u16, &'static str);

    #[test]
    fn test_golden() {
        let golden = [
            TestCase(0x0000, ""),
            TestCase(0xe8c1, "a"),
            TestCase(0x79a8, "ab"),
            TestCase(0x9738, "abc"),
            TestCase(0x3997, "abcd"),
            TestCase(0x85b8, "abcde"),
            TestCase(0x5805, "abcdef"),
            TestCase(0xe9d9, "abcdefg"),
            TestCase(0x7429, "abcdefgh"),
            TestCase(0xf075, "abcdefghi"),
            TestCase(0xc8b1, "abcdefghij"),
            TestCase(0x2ea0, "Discard medicine more than two years old."),
            TestCase(
                0x276b,
                "He who has a shady past knows that nice guys finish last.",
            ),
            TestCase(0x1abb, "I wouldn't marry him with a ten foot pole."),
            TestCase(
                0x9499,
                "Free! Free!/A trip/to Mars/for 900/empty jars/Burma Shave",
            ),
            TestCase(
                0xabfd,
                "The days of the digital watch are numbered.  -Tom Stoppard",
            ),
            TestCase(0x4ee5, "Nepal premier won't resign."),
            TestCase(
                0x761c,
                "For every action there is an equal and opposite government \
                 program.",
            ),
            TestCase(
                0xb823,
                "His money is twice tainted: 'taint yours and 'taint mine.",
            ),
            TestCase(
                0xd283,
                "There is no reason for any individual to have a computer in \
                 their home. -Ken Olsen, 1977",
            ),
            TestCase(
                0x364a,
                "It's a tiny change to the code and not completely \
                 disgusting. - Bob Manchek",
            ),
            TestCase(0x657f, "size:  a.out:  bad magic"),
            TestCase(
                0xe8ec,
                "The major problem is with sendmail.  -Mark Horton",
            ),
            TestCase(
                0xbdb9,
                "Give me a rock, paper and scissors and I wi)l move the \
                 world.  CCFestoon",
            ),
            TestCase(0x3032, "If the enemy is within range, then so are you."),
            TestCase(
                0xc114,
                "It's well we cannot hear the screams/That we create in \
                 others' dreams.",
            ),
            TestCase(
                0x161f,
                "You remind me of a TV show, but that's all right: I watch it \
                 anyway.",
            ),
            TestCase(0x12c6, "C is as portable as Stonehedge!!"),
            TestCase(
                0xc633,
                "Even if I could be Shakespeare, I think I should still \
                 choose to be Faraday. - A. Huxley",
            ),
            TestCase(
                0xf768,
                "The fugacity of a constituent in a mixture of gases at a \
                 given temperature is proportional to its mole fraction.  \
                 Lewis-Randall Rule",
            ),
            TestCase(
                0xbcef,
                "How can you write a big system without C++?  -Paul Glick",
            ),
        ];

        for &TestCase(want, input) in &golden {
            let got = checksum(input.as_bytes());
            assert_eq!(want, got)
        }
    }

    #[test]
    fn test_running_checksum() {
        let golden_running = [
            TestCase(0x0000, ""),
            TestCase(0x79a8, "ab"),
            TestCase(0xe79a, "abc"),
            TestCase(0xd609, "abcd"),
            TestCase(0xcc5c, "abcde"),
            TestCase(0xd419, "abcdef"),
            TestCase(0x3c43, "abcdefg"),
            TestCase(0x5291, "abcdefgh"),
            TestCase(0x8350, "abcdefghi"),
            TestCase(0xbfc7, "abcdefghij"),
            TestCase(0x122d, "Discard medicine more than two years old."),
            TestCase(
                0xd6e7,
                "He who has a shady past knows that nice guys finish last.",
            ),
            TestCase(0x14aa, "I wouldn't marry him with a ten foot pole."),
            TestCase(
                0xf34f,
                "Free! Free!/A trip/to Mars/for 900/empty jars/Burma Shave",
            ),
            TestCase(
                0x9a61,
                "The days of the digital watch are numbered.  -Tom Stoppard",
            ),
            TestCase(0xaf1d, "Nepal premier won't resign."),
            TestCase(
                0xf690,
                "For every action there is an equal and opposite government \
                 program.",
            ),
            TestCase(
                0xd7c8,
                "His money is twice tainted: 'taint yours and 'taint mine.",
            ),
            TestCase(
                0x36cf,
                "There is no reason for any individual to have a computer in \
                 their home. -Ken Olsen, 1977",
            ),
            TestCase(
                0xd677,
                "It's a tiny change to the code and not completely \
                 disgusting. - Bob Manchek",
            ),
            TestCase(0xb5b5, "size:  a.out:  bad magic"),
            TestCase(
                0xc519,
                "The major problem is with sendmail.  -Mark Horton",
            ),
            TestCase(
                0x9846,
                "Give me a rock, paper and scissors and I wi)l move the \
                 world.  CCFestoon",
            ),
            TestCase(0xba06, "If the enemy is within range, then so are you."),
            TestCase(
                0x441e,
                "It's well we cannot hear the screams/That we create in \
                 others' dreams.",
            ),
            TestCase(
                0x4bf3,
                "You remind me of a TV show, but that's all right: I watch it \
                 anyway.",
            ),
            TestCase(0x8781, "C is as portable as Stonehedge!!"),
            TestCase(
                0x13f4,
                "Even if I could be Shakespeare, I think I should still \
                 choose to be Faraday. - A. Huxley",
            ),
            TestCase(
                0x5ba4,
                "The fugacity of a constituent in a mixture of gases at a \
                 given temperature is proportional to its mole fraction.  \
                 Lewis-Randall Rule",
            ),
            TestCase(
                0x8008,
                "How can you write a big system without C++?  -Paul Glick",
            ),
        ];

        let mut h = CRC16::new();
        for &TestCase(want, input) in &golden_running {
            h.write(input.as_bytes()).expect("can't fail");
            let got = h.sum_16();
            assert_eq!(want, got)
        }
    }

    // Returns the Dynastream CRC-16 checksum of data.
    fn checksum(data: &[u8]) -> u16 {
        let mut crc = CRC16(0);
        crc.update(data);
        crc.sum_16()
    }
}
