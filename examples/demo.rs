use autotokenizer::{AutoTokenizer, DefaultPromptMessage};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let a = AutoTokenizer::from_pretrained("TinyLlama/TinyLlama-1.1B-Chat-v1.0".to_string(), None)?;
    let ctx = vec![
        DefaultPromptMessage::new("system", "Hello"),
        DefaultPromptMessage::new("user", "Hello"),
        DefaultPromptMessage::new("assistant", "Hello"),
    ];
    let chat = a.apply_chat_template(ctx, true)?;
    println!("this is chat:{}", chat);
    Ok(())
}
