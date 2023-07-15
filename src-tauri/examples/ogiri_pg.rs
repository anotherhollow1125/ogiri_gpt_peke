use anyhow::Result;
use dialoguer::Input;
use ogiri_gpt_peke::openai_structs::{Message, Role};
use ogiri_gpt_peke::{balthasar_interact, casper_interact, melchior_interact};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY")?;

    let mut messages = vec![];

    loop {
        let input = Input::new()
            .with_prompt("お題")
            .interact_text()
            .unwrap_or_else(|_| "quit".to_string());

        if input == "quit" {
            break;
        }

        messages.push(Message {
            role: Role::User,
            content: input,
        });

        let mel_response = melchior_interact(&api_key, &messages).await?;
        println!("{:?}", mel_response);
        messages.push(mel_response);

        let bal_response = balthasar_interact(&api_key, &messages).await?;
        println!("{:?}", bal_response);
        messages.push(bal_response);

        let cas_response = casper_interact(&api_key, &messages).await?;
        println!("{:?}", cas_response);
        messages.push(cas_response);

        println!("===\n{:?}", messages);
    }

    Ok(())
}
