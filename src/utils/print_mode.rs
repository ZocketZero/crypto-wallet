#[derive(Clone, Debug, clap::ValueEnum)]
pub enum PrintMode {
    A,
    All,
    S,
    Secret,
    P,
    Public,
    Json,
}

impl std::str::FromStr for PrintMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" | "all" => Ok(PrintMode::All),
            "s" | "secret" => Ok(PrintMode::Secret),
            "p" | "public" => Ok(PrintMode::Public),
            "json" => Ok(PrintMode::Json),
            _ => Err(format!("Invalid print mode: {}", s)),
        }
    }
}
impl PrintMode {
    pub fn is_all(&self) -> bool {
        matches!(self, Self::A | Self::All)
    }
    pub fn is_secret(&self) -> bool {
        matches!(self, Self::A | Self::All | Self::S | Self::Secret)
    }
    pub fn is_public(&self) -> bool {
        matches!(self, Self::A | Self::All | Self::P | Self::Public)
    }
    pub fn is_json(&self) -> bool {
        matches!(self, Self::Json)
    }
}
