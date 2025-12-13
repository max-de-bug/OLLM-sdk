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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_as_str() {
        assert_eq!(Model::NearGLM46.as_str(), "near/GLM-4.6");
    }

    #[test]
    fn test_model_equality() {
        assert_eq!(Model::NearGLM46, Model::NearGLM46);
    }

    #[test]
    fn test_model_clone() {
        let model = Model::NearGLM46;
        let cloned = model.clone();
        assert_eq!(model, cloned);
    }
}
