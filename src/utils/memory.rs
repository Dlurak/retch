#[derive(Copy, Clone)]
enum MemoryUnit {
    Byte,
    KiloByte,
    MegaByte,
    GigaByte,
    TerraByte,
    PetaByte
}

impl MemoryUnit {
    fn next_bigger(&self) -> Option<MemoryUnit> {
        match *self {
            MemoryUnit::Byte => Some(MemoryUnit::KiloByte),
            MemoryUnit::KiloByte => Some(MemoryUnit::MegaByte),
            MemoryUnit::MegaByte => Some(MemoryUnit::GigaByte),
            MemoryUnit::GigaByte => Some(MemoryUnit::TerraByte),
            MemoryUnit::TerraByte => Some(MemoryUnit::PetaByte),
            MemoryUnit::PetaByte => None,
        }
    }

    fn abbr(&self) -> &str {
        match *self {
            MemoryUnit::Byte => "B",
            MemoryUnit::KiloByte => "KB",
            MemoryUnit::MegaByte => "MB",
            MemoryUnit::GigaByte => "GB",
            MemoryUnit::TerraByte => "TB",
            MemoryUnit::PetaByte => "PB",
        }
    }
}

#[derive(Copy, Clone)]
pub struct Memory {
    pub value: f64,
    unit: MemoryUnit
}

impl Memory {
    pub fn new(value: f64) -> Memory {
        Memory {
            value,
            unit: MemoryUnit::Byte,
        }
    }

    pub fn human_size(&self) -> Memory {
        if self.value < 1024.0 {
            return *self;
        }

        match self.unit.next_bigger() {
            Some(unit) => Memory {
                    value: self.value / 1024.0,
                    unit
                }.human_size(),
            None => *self
        }
    }

    pub fn format(&self) -> String {
        format!(
            "{:.2}{}",
            self.value,
            self.unit.abbr()
        )
    }
}
