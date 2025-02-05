/// An photon incidence event, consisting of location, energy and timestamp
#[derive(Debug, PartialEq, Eq)]
pub struct Event {
    /// The bit depth of the x & y locations within the id
    bitdepth: u32,
    /// The location of the event on the x axis of the detector
    x: u16,
    /// The location of the event on the y axis of the detector
    y: u16,
    /// The energy of the incident photon
    energy: u32,
    /// The timestamp of the event
    time: u64,
}

/// The raw binary representation of fields in an [`Event`]
#[derive(Debug, PartialEq, Eq)]
pub struct EventRaw {
    /// The bit depth of the x & y locations within the id
    bitdepth: u32,
    /// The id, containing an x & y location of the event, with variable bit depth and packed in
    /// little endian
    id: [u8; 4],
    /// The energy of the incident photon, as a little endian unsigned 32 bit integer
    energy: [u8; 4],
    /// The timestamp of the event, as a little endian unsigned 64 bit integer
    time: [u8; 8],
}

impl From<EventRaw> for Event {
    fn from(value: EventRaw) -> Self {
        let id = u32::from_le_bytes(value.id);
        let y = (id & ((1 << value.bitdepth) - 1)) as u16;
        let x = ((id & (((1 << value.bitdepth) - 1) << value.bitdepth)) >> value.bitdepth) as u16;
        Self {
            bitdepth: value.bitdepth,
            x,
            y,
            energy: u32::from_le_bytes(value.energy),
            time: u64::from_le_bytes(value.time),
        }
    }
}

impl From<Event> for EventRaw {
    fn from(value: Event) -> Self {
        let mut id = 0;
        id |= value.y as u32;
        id |= (value.x as u32) << value.bitdepth;
        Self {
            bitdepth: value.bitdepth,
            id: id.to_le_bytes(),
            energy: value.energy.to_le_bytes(),
            time: value.time.to_le_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, EventRaw};

    #[test]
    fn deserialize_raw_event() {
        let raw = EventRaw {
            bitdepth: 13,
            id: [0x45, 0x80, 0x34, 0x00],
            energy: [0xD2, 0x04, 0x00, 0x00],
            time: [0x2E, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        };
        let expected = Event {
            bitdepth: 13,
            x: 420,
            y: 69,
            energy: 1234,
            time: 5678,
        };
        assert_eq!(expected, Event::from(raw))
    }

    #[test]
    fn serialize_event() {
        let event = Event {
            bitdepth: 13,
            x: 420,
            y: 69,
            energy: 1234,
            time: 5678,
        };
        let expected = EventRaw {
            bitdepth: 13,
            id: [0x45, 0x80, 0x34, 0x00],
            energy: [0xD2, 0x04, 0x00, 0x00],
            time: [0x2E, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        };
        assert_eq!(expected, EventRaw::from(event))
    }
}
