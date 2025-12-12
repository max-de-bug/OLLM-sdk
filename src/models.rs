pub enum Model {
    NearGLM46,
    GPT4Mini,
    GeminiFlash,
    // ...
}

impl Model {
    pub fn as_str(&self) -> &'static str {
        match self {
            Model::NearGLM46 => "near/GLM-4.6",
            Model::GPT4Mini => "gpt-4.1-mini",
            Model::GeminiFlash => "gemini/flash",
        }
    }
}
