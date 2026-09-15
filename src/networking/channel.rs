use std::fmt;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Channel {
    pub frequency: u32,
    pub ir: bool,
    pub radar_detection: bool,
}

impl Channel {
    pub fn new(frequency: u32, ir: bool, radar_detection: bool) -> Self {
        Channel {
            frequency: frequency,
            ir: ir,
            radar_detection: radar_detection,
        }
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "Freq {}{}{}",
            self.frequency,
            if self.ir { "(IR)" } else { "" },
            if self.radar_detection { "RADAR" } else { "" }
        );
    }
}
