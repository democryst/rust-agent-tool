use crate::domain::Chunk;

pub struct PromptTransformer;

impl PromptTransformer {
    pub fn transform(user_input: &str, context: Vec<Chunk>) -> String {
        let mut prompt = String::from("<start_of_turn>user\n");
        prompt.push_str("You are an Elite AI Architect running on native hardware. ");
        prompt.push_str("Grounded by the following context, provide a precise, technical, and brief answer.\n\n");
        
        prompt.push_str("CONTEXT:\n");
        for chunk in context {
            prompt.push_str(&format!("- {}\n", chunk.content));
        }
        
        prompt.push_str("\nQUERY:\n");
        prompt.push_str(user_input);
        prompt.push_str("<end_of_turn>\n<start_of_turn>model\n");
        
        prompt
    }
}
