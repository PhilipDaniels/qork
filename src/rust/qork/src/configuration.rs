
// Stores the configuration.

pub struct Configuration {
    num_mru_items: u32
}

impl Configuration {
    pub fn default() -> Configuration {
        Configuration {
            num_mru_items: 20
        }
    }
}