use num_enum::{IntoPrimitive, TryFromPrimitive};

/// A metadata esque cue event recorded by the detector
#[derive(Debug, PartialEq, Eq)]
pub struct Cue {
    /// An identifier describing the nature of the cue
    id: CueId,
    /// The timestamp of the cue
    time: u64,
}

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum CueId {
    Padding = 0x0000,
    ExtendedTimestamp = 0x0800,
    ShutterOpen = 0x0840,
    ShutterClose = 0x0880,
    FemFalling = 0x08C1,
    FemRising = 0x08E1,
    TtlFalling = 0x08C9,
    TtlRising = 0x08E9,
    LvdsFalling = 0x0CA,
    LvdsRising = 0x08EA,
    TZeroFalling = 0x08CB,
    TZeroRising = 0x08EB,
    SyncFalling = 0x08CC,
    SyncRising = 0x08EC,
    Reserved = 0x0F00,
}

/// The raw binary representatiuon of fields in a [`Cue`]
#[derive(Debug, PartialEq, Eq)]
pub struct CueRaw {
    /// An identifier describing the nature of the cue, as a little endian unsigned 16 bit
    /// integer
    id: [u8; 2],
    /// The timestamp of the cue, as a little endian unsigned 64 bit integer
    time: [u8; 8],
}

impl TryFrom<CueRaw> for Cue {
    type Error = num_enum::TryFromPrimitiveError<CueId>;

    fn try_from(value: CueRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            id: u16::from_le_bytes(value.id).try_into()?,
            time: u64::from_le_bytes(value.time),
        })
    }
}

impl From<Cue> for CueRaw {
    fn from(value: Cue) -> Self {
        Self {
            id: u16::from(value.id).to_le_bytes(),
            time: value.time.to_le_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cue, CueId, CueRaw};

    #[test]
    fn deserialize_raw_cue() {
        let raw = CueRaw {
            id: [0x00, 0x08],
            time: [0x2E, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        };
        let expected = Cue {
            id: CueId::ExtendedTimestamp,
            time: 5678,
        };
        assert_eq!(expected, raw.try_into().unwrap())
    }

    #[test]
    fn serialize_cue() {
        let cue = Cue {
            id: CueId::ExtendedTimestamp,
            time: 5678,
        };
        let expected = CueRaw {
            id: [0x00, 0x08],
            time: [0x2E, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        };
        assert_eq!(expected, cue.into())
    }
}
