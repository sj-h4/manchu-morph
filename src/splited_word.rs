pub struct SplitedWord {
    base: String,
    suffix: Option<String>,
}

impl SplitedWord {
    pub fn new(base: String, suffix: Option<String>) -> Self {
        Self { base, suffix }
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }
}

