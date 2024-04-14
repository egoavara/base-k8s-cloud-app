use derivative::Derivative;

pub const DEFAULT_CONFIG: Config = Config {
    sql: OptionsSql { default_limit: 1000, max_limit: None },
    processing: OptionsProcessing { chunking: Some(Chunking::Auto) },
};
#[derive(Debug, Clone, Derivative, serde::Serialize, serde::Deserialize)]
#[derivative(Default)]
pub struct Config {
    pub sql: OptionsSql,

    pub processing: OptionsProcessing,
}

#[derive(Debug, Clone, Derivative, serde::Serialize, serde::Deserialize)]
#[derivative(Default)]
pub struct OptionsSql {
    #[derivative(Default(value = "1000"))]
    default_limit: u32,
    max_limit: Option<u32>,
}
#[derive(Debug, Clone, Derivative, serde::Serialize, serde::Deserialize)]
#[derivative(Default)]
pub struct OptionsProcessing {
    pub chunking: Option<Chunking>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Chunking {
    None,
    Auto,
    ChunkSize(u32),
}

impl OptionsProcessing {
    pub fn chunking_size(&self) -> Option<u32> {
        match self.chunking {
            Some(Chunking::ChunkSize(size)) => Some(size),
            Some(Chunking::Auto) => Some(1000),
            _ => None,
        }
    }
}
