use anyhow::Result;
use dialoguer::Input;
use ogiri_gpt_peke::gpt_base;
use ogiri_gpt_peke::openai_structs::{Message, Role};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY")?;

    let mut messages = vec![Message {
        role: Role::System,
        content: "You are a helpful assistant.".to_string(),
    }];

    loop {
        let input = Input::new()
            .with_prompt("You")
            .interact_text()
            .unwrap_or_else(|_| "quit".to_string());

        if input == "quit" {
            break;
        }

        messages.push(Message {
            role: Role::User,
            content: input,
        });

        let response = gpt_base(&api_key, &messages).await?;

        println!("{}", response.content);

        messages.push(response);
    }

    Ok(())
}
