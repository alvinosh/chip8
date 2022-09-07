pub struct DebugLevels {
    pub log_all: bool,
    pub step: bool,
}

impl Default for DebugLevels {
    fn default() -> Self {
        Self {
            log_all: false,
            step: false,
        }
    }
}
