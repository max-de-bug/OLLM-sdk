#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Model {
    NearGLM46,
}

impl Model {
    pub fn as_str(&self) -> &str {
        match self {
            Model::NearGLM46 => "near/GLM-4.6",
        }
    }
}
