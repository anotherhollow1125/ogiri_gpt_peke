#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ogiri_gpt_peke::openai_structs::{Message as APIMessage, Role};
use ogiri_gpt_peke::{balthasar_interact, casper_interact, melchior_interact};
use tokio::sync::mpsc::{channel, Sender};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
enum Character {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "melchior")]
    Melchior,
    #[serde(rename = "balthasar")]
    Balthasar,
    #[serde(rename = "casper")]
    Casper,
}

use Character::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Message {
    character: Character,
    content: String,
}

fn message_2_api_message(message: &Message) -> APIMessage {
    APIMessage {
        role: match message.character {
            User => Role::User,
            Melchior => Role::Assistant,
            Balthasar => Role::Assistant,
            Casper => Role::Assistant,
        },
        content: message.content.clone(),
    }
}

type MessageHistory = Vec<APIMessage>;

fn append_history_and_get_context(
    history: &mut MessageHistory,
    context: Vec<Message>,
) -> Vec<APIMessage> {
    // dummy 挿入処理

    if (history.len() > 0)
        && (context.len() > 0)
        && (history[history.len() - 1].role == Role::User)
        && (context[0].character == User)
    {
        history.push(APIMessage {
            role: Role::Assistant,
            content: "".to_string(),
        });
    }

    history.extend(
        context
            .iter()
            .map(message_2_api_message)
            .collect::<Vec<_>>(),
    );
    history.clone()
}

type SenderSet = (Vec<Message>, Sender<Result<Message, String>>);

struct MelchiorSender(Sender<SenderSet>);

fn init_melchior_thread(api_key: String) -> MelchiorSender {
    let (input_tx, mut input_rx) = channel::<SenderSet>(32);

    tokio::spawn(async move {
        let mut history = MessageHistory::new();

        while let Some((context, output_tx)) = input_rx.recv().await {
            let m = append_history_and_get_context(&mut history, context);

            println!("ask melchior: {:?}", m);

            let response = melchior_interact(&api_key, &m).await.map_err(|e| {
                eprintln!("[Melchior Error] {:?}", e);
                e.to_string()
            });

            let response = match response {
                Ok(r) => r,
                Err(e) => {
                    output_tx.send(Err(e)).await.unwrap();
                    continue;
                }
            };

            history.push(response.clone());

            output_tx
                .send(Ok(Message {
                    character: Melchior,
                    content: response.content,
                }))
                .await
                .unwrap();
        }
    });

    MelchiorSender(input_tx)
}

#[tauri::command]
async fn melchior(
    sender: tauri::State<'_, MelchiorSender>,
    context: Vec<Message>,
) -> Result<Message, String> {
    let (output_tx, mut output_rx) = channel(1);

    sender
        .0
        .send((context, output_tx))
        .await
        .map_err(|e| format!("{:?}", e))?;

    let res = output_rx
        .recv()
        .await
        .ok_or_else(|| "no response".to_string())??;

    Ok(res)
}

struct BalthasarSender(Sender<SenderSet>);

fn init_balthasar_thread(api_key: String) -> BalthasarSender {
    let (input_tx, mut input_rx) = channel::<SenderSet>(32);

    tokio::spawn(async move {
        let mut history = MessageHistory::new();

        while let Some((context, output_tx)) = input_rx.recv().await {
            let m = append_history_and_get_context(&mut history, context);

            println!("ask balthasar: {:?}", m);

            let response = balthasar_interact(&api_key, &m).await.map_err(|e| {
                eprintln!("[Balthasar Error] {:?}", e);
                e.to_string()
            });

            let response = match response {
                Ok(r) => r,
                Err(e) => {
                    output_tx.send(Err(e)).await.unwrap();
                    continue;
                }
            };

            history.push(response.clone());

            output_tx
                .send(Ok(Message {
                    character: Balthasar,
                    content: response.content,
                }))
                .await
                .unwrap();
        }
    });

    BalthasarSender(input_tx)
}

#[tauri::command]
async fn balthasar(
    sender: tauri::State<'_, BalthasarSender>,
    context: Vec<Message>,
) -> Result<Message, String> {
    let (output_tx, mut output_rx) = channel(1);

    sender
        .0
        .send((context, output_tx))
        .await
        .map_err(|e| format!("{:?}", e))?;

    let res = output_rx
        .recv()
        .await
        .ok_or_else(|| "no response".to_string())??;

    Ok(res)
}

struct CasperSender(Sender<SenderSet>);

fn init_casper_thread(api_key: String) -> CasperSender {
    let (input_tx, mut input_rx) = channel::<SenderSet>(32);

    tokio::spawn(async move {
        let mut history = MessageHistory::new();

        while let Some((context, output_tx)) = input_rx.recv().await {
            let m = append_history_and_get_context(&mut history, context);

            println!("ask casper: {:?}", m);

            let response = casper_interact(&api_key, &m).await.map_err(|e| {
                eprintln!("[Casper Error] {:?}", e);
                e.to_string()
            });

            let response = match response {
                Ok(r) => r,
                Err(e) => {
                    output_tx.send(Err(e)).await.unwrap();
                    continue;
                }
            };

            history.push(response.clone());

            output_tx
                .send(Ok(Message {
                    character: Casper,
                    content: response.content,
                }))
                .await
                .unwrap();
        }
    });

    CasperSender(input_tx)
}

#[tauri::command]
async fn casper(
    sender: tauri::State<'_, CasperSender>,
    context: Vec<Message>,
) -> Result<Message, String> {
    let (output_tx, mut output_rx) = channel(1);

    sender
        .0
        .send((context, output_tx))
        .await
        .map_err(|e| format!("{:?}", e))?;

    let res = output_rx
        .recv()
        .await
        .ok_or_else(|| "no response".to_string())??;

    Ok(res)
}

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    dotenv::dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let melchior_sender = init_melchior_thread(api_key.clone());
    let balthasar_sender = init_balthasar_thread(api_key.clone());
    let casper_sender = init_casper_thread(api_key.clone());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![melchior, balthasar, casper])
        .manage(melchior_sender)
        .manage(balthasar_sender)
        .manage(casper_sender)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
