pub mod openai_structs;

use anyhow::Result;
use openai_structs::{Message, RequestBody, Role};
use reqwest::{Client, RequestBuilder};

pub fn common_header(api_key: &str) -> RequestBuilder {
    let api_key_field = format!("Bearer {}", api_key);

    Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", api_key_field.as_str())
}

pub async fn gpt_base(api_key: &str, input_messages: &[Message]) -> Result<Message> {
    let rb = common_header(api_key)
        .json(&RequestBody {
            model: "gpt-3.5-turbo".to_string(),
            messages: Vec::from(input_messages),
        })
        .send()
        .await?
        .json::<openai_structs::ResponseBody>()
        .await?;

    Ok(rb.choices[0].message.clone())
}

fn melchior_init_prompt() -> Message {
    Message {
        role: Role::System,
        content: "大喜利という、与えられたお題に対して可笑しくて笑ってしまうような回答を返す遊びがあります。\
            これから User が大喜利のお題を\"お題「〇〇」\"という形であなたに渡すので、お題に沿った面白く洒落の効いた回答を考え、\
            「Melchiorが回答します。」と名乗った後に、回答のみを1文で簡潔に返答してください。\
            ただしここまでのすべてのやり取りの中で同じお題が2回出題されることがあります。\
            その際は大喜利への回答はせず、\
            「Melchiorが回答します。何故同じお題が与えられたのですか?」と聞き返してください。".to_string(),
    }
}

pub async fn melchior_interact(api_key: &str, context: &[Message]) -> Result<Message> {
    let mut messages = vec![melchior_init_prompt()];
    messages.extend_from_slice(context);

    gpt_base(api_key, &messages).await
}

fn balthasar_init_prompt() -> Message {
    Message {
        role: Role::System,
        content: "大喜利という、与えられたお題に対して可笑しくて笑ってしまうような回答を返す遊びがあります。\
            今、大喜利を行っており、あなたの前に Melchior という別なアシスタントが User が出したお題に沿った面白い回答を返してきます。 \
            Melchiorの回答と被らないように注意しつつ、お題に沿った皮肉の効いたブラックジョークな回答を考え、\
            「Balthasarが回答します。」と名乗った後に、回答のみを1文で簡潔に返答してください。".to_string(),
    }
}

/*
ただしここまでのすべてのやり取りの中で同じお題が2回出題されることがあり、\
そのことをMelchiorが指摘しています。\
その際は「Balthasarが回答します。」名乗った後に、大喜利への回答はせず、\
「何故同じお題を出したのですか?」と聞き返してください。
*/

pub async fn balthasar_interact(api_key: &str, context: &[Message]) -> Result<Message> {
    let mut messages = vec![balthasar_init_prompt()];
    messages.extend_from_slice(context);

    gpt_base(api_key, &messages).await
}

fn casper_init_prompt() -> Message {
    Message {
        role: Role::System,
        content: "大喜利という、与えられたお題に対して可笑しくて笑ってしまうような回答を返す遊びがあります。\
        今、大喜利を行っており、あなたの前に Melchior と Balthasar という2人の別なアシスタントが User が出したお題に沿った面白い回答を返してきます。 \
        Melchior と Balthasar の回答と被らないように注意しつつ、お題からは考えつかないような意外な回答を考え、\
        「Casperが回答します。」と名乗った後に、回答のみを1文で簡潔に返答してください。".to_string(),
    }
}

/*
ただしここまでのすべてのやり取りの中で同じお題が2回出題されることがあり、\
そのことを他の回答者が指摘しています。\
その際は「Casperが回答します。」と名乗った後に、大喜利への回答はせず、\
「何故同じお題を出したのですか?」と聞き返してください。
*/

pub async fn casper_interact(api_key: &str, context: &[Message]) -> Result<Message> {
    let mut messages = vec![casper_init_prompt()];
    messages.extend_from_slice(context);

    gpt_base(api_key, &messages).await
}
