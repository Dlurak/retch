use std::fmt;

#[derive(Copy, Clone)]
enum MemoryUnit {
    Byte,
    KiloByte,
    MegaByte,
    GigaByte,
    TerraByte,
    PetaByte,
}

impl MemoryUnit {
    fn next_bigger(&self) -> Option<Self> {
        match *self {
            MemoryUnit::Byte => Some(Self::KiloByte),
            MemoryUnit::KiloByte => Some(Self::MegaByte),
            MemoryUnit::MegaByte => Some(Self::GigaByte),
            MemoryUnit::GigaByte => Some(Self::TerraByte),
            MemoryUnit::TerraByte => Some(Self::PetaByte),
            MemoryUnit::PetaByte => None,
        }
    }

    fn abbr(&self) -> &str {
        match *self {
            Self::Byte => "B",
            Self::KiloByte => "KB",
            Self::MegaByte => "MB",
            Self::GigaByte => "GB",
            Self::TerraByte => "TB",
            Self::PetaByte => "PB",
        }
    }
}

#[derive(Copy, Clone)]
pub struct Memory {
    pub value: f64,
    unit: MemoryUnit,
}

impl Memory {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            unit: MemoryUnit::Byte,
        }
    }

    pub fn human_size(&self) -> Self {
        if self.value < 1024.0 {
            return *self;
        }

        self.unit.next_bigger().map_or(*self, |unit| {
            Self {
                value: self.value / 1024.0,
                unit,
            }
            .human_size()
        })
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}{}", self.value, self.unit.abbr())
    }
}
