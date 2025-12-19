#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Model {
    // NEAR Provider Models
    NearDeepSeekV31,
    NearGLM46,
    NearGptOss120b,
    NearQwen330BA3BInstruct2507,
    // Phala Provider Models
    PhalaDeepSeekChatV30324,
    PhalaDeepSeekChatV31,
    PhalaDeepSeekR10528,
    PhalaGemma327bIt,
    PhalaGLM46,
    PhalaGptOss120b,
}

impl Model {
    pub fn as_str(&self) -> &str {
        match self {
            // NEAR Provider Models
            Model::NearDeepSeekV31 => "near/DeepSeek-V3.1",
            Model::NearGLM46 => "near/GLM-4.6",
            Model::NearGptOss120b => "near/gpt-oss-120b",
            Model::NearQwen330BA3BInstruct2507 => "near/Qwen3-30B-A3B-Instruct-2507",
            // Phala Provider Models
            Model::PhalaDeepSeekChatV30324 => "phala/deepseek-chat-v3-0324",
            Model::PhalaDeepSeekChatV31 => "phala/deepseek-chat-v3.1",
            Model::PhalaDeepSeekR10528 => "phala/deepseek-r1-0528",
            Model::PhalaGemma327bIt => "phala/gemma-3-27b-it",
            Model::PhalaGLM46 => "phala/glm-4.6",
            Model::PhalaGptOss120b => "phala/gpt-oss-120b",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_models_as_str() {
        // Test all NEAR Provider models
        assert_eq!(Model::NearDeepSeekV31.as_str(), "near/DeepSeek-V3.1");
        assert_eq!(Model::NearGLM46.as_str(), "near/GLM-4.6");
        assert_eq!(Model::NearGptOss120b.as_str(), "near/gpt-oss-120b");
        assert_eq!(
            Model::NearQwen330BA3BInstruct2507.as_str(),
            "near/Qwen3-30B-A3B-Instruct-2507"
        );

        // Test all Phala Provider models
        assert_eq!(
            Model::PhalaDeepSeekChatV30324.as_str(),
            "phala/deepseek-chat-v3-0324"
        );
        assert_eq!(
            Model::PhalaDeepSeekChatV31.as_str(),
            "phala/deepseek-chat-v3.1"
        );
        assert_eq!(
            Model::PhalaDeepSeekR10528.as_str(),
            "phala/deepseek-r1-0528"
        );
        assert_eq!(Model::PhalaGemma327bIt.as_str(), "phala/gemma-3-27b-it");
        assert_eq!(Model::PhalaGLM46.as_str(), "phala/glm-4.6");
        assert_eq!(Model::PhalaGptOss120b.as_str(), "phala/gpt-oss-120b");
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
