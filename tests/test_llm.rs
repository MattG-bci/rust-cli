#[cfg(test)]
mod tests {
    use ollama_rs::generation::completion::request::GenerationRequest;
    use ollama_rs::generation::completion::GenerationResponse;
    use rust_cli::llm::cli::{
        concat_text_file_and_command, get_prompt_initial_message, LLMCommand,
    };

    struct MockOllama;
    impl MockOllama {
        async fn generate(
            &self,
            _request: GenerationRequest<'_>,
        ) -> Result<GenerationResponse, Box<dyn std::error::Error>> {
            Ok(GenerationResponse {
                model: "".to_string(),
                created_at: "".to_string(),
                response: "test response".to_string(),
                done: false,
                context: None,
                total_duration: None,
                load_duration: None,
                prompt_eval_count: None,
                prompt_eval_duration: None,
                eval_count: None,
                eval_duration: None,
                thinking: None,
            })
        }
    }

    pub async fn generate_response_testable(
        params: &LLMCommand,
        ollama: &MockOllama,
        text: String,
    ) -> Result<GenerationResponse, Box<dyn std::error::Error>> {
        let initial_message = get_prompt_initial_message(&params.command)?;
        let prompt = concat_text_file_and_command(&initial_message, &text);
        let request = GenerationRequest::new(params.model.clone(), prompt);
        if let true = params.with_thinking {
            let request_with_thinking = request.clone().think(true);
            let response = ollama.generate(request_with_thinking).await?;
            Ok(response)
        } else {
            let response = ollama.generate(request).await?;
            Ok(response)
        }
    }

    #[tokio::test]
    async fn test_generate_response_without_thinking() {
        let mock_ollama = MockOllama;

        let params = LLMCommand {
            command: "explain".to_string(),
            model: "test-model".to_string(),
            with_thinking: false,
            path_to_file: "".to_string(),
        };

        let result =
            generate_response_testable(&params, &mock_ollama, "input text".to_string()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().response, "test response");
    }
}
