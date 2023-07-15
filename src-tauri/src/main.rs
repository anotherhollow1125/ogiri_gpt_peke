#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ogiri_gpt_peke::openai_structs::{Message as APIMessage, Role};
use ogiri_gpt_peke::{balthasar_interact, casper_interact, melchior_interact};
use std::sync::Mutex;

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

#[derive(Debug, Clone)]
struct MelchiorHistory(MessageHistory);

#[derive(Debug, Clone)]
struct MelchCounter(u64);

#[derive(Debug, Clone)]
struct BalthasarHistory(MessageHistory);

#[derive(Debug, Clone)]
struct BalthCounter(u64);

#[derive(Debug, Clone)]
struct CasperHistory(MessageHistory);

#[derive(Debug, Clone)]
struct CasperCounter(u64);

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

fn get_count(delay_counter: &mut u64) -> u64 {
    let now = *delay_counter;
    *delay_counter += 1;
    now
}

#[tauri::command]
async fn melchior(
    api_key: tauri::State<'_, String>,
    history: tauri::State<'_, Mutex<MelchiorHistory>>,
    delay_counter: tauri::State<'_, Mutex<MelchCounter>>,
    context: Vec<Message>,
) -> Result<Message, String> {
    let count = {
        let mut delay_counter = delay_counter.lock().unwrap();
        get_count(&mut delay_counter.0)
    };

    let m = {
        let mut history = history.lock().unwrap();
        append_history_and_get_context(&mut history.0, context)
    };

    println!("ask melchior: {:?}", m);

    let response = melchior_interact(&api_key, &m)
        .await
        .map_err(|e| e.to_string())?;

    {
        let mut history = history.lock().unwrap();
        history.0.push(response.clone());
    }

    tokio::time::sleep(std::time::Duration::from_millis(3000 * count)).await;

    Ok(Message {
        character: Melchior,
        content: response.content,
    })
}

#[tauri::command]
async fn balthasar(
    api_key: tauri::State<'_, String>,
    history: tauri::State<'_, Mutex<BalthasarHistory>>,
    delay_counter: tauri::State<'_, Mutex<BalthCounter>>,
    context: Vec<Message>,
) -> Result<Message, String> {
    let count = {
        let mut delay_counter = delay_counter.lock().unwrap();
        get_count(&mut delay_counter.0)
    };

    let m = {
        let mut history = history.lock().unwrap();
        append_history_and_get_context(&mut history.0, context)
    };

    println!("ask balthasar: {:?}", m);

    let response = balthasar_interact(&api_key, &m)
        .await
        .map_err(|e| e.to_string())?;

    {
        let mut history = history.lock().unwrap();
        history.0.push(response.clone());
    }

    tokio::time::sleep(std::time::Duration::from_millis(3000 * count)).await;

    Ok(Message {
        character: Balthasar,
        content: response.content,
    })
}

#[tauri::command]
async fn casper(
    api_key: tauri::State<'_, String>,
    history: tauri::State<'_, Mutex<CasperHistory>>,
    delay_counter: tauri::State<'_, Mutex<CasperCounter>>,
    context: Vec<Message>,
) -> Result<Message, String> {
    let count = {
        let mut delay_counter = delay_counter.lock().unwrap();
        get_count(&mut delay_counter.0)
    };

    let m = {
        let mut history = history.lock().unwrap();
        append_history_and_get_context(&mut history.0, context)
    };

    println!("ask casper: {:?}", m);

    let response = casper_interact(&api_key, &m)
        .await
        .map_err(|e| e.to_string())?;

    {
        let mut history = history.lock().unwrap();
        history.0.push(response.clone());
    }

    tokio::time::sleep(std::time::Duration::from_millis(3000 * count)).await;

    Ok(Message {
        character: Casper,
        content: response.content,
    })
}

fn main() {
    dotenv::dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let melchior_history: Vec<APIMessage> = vec![];
    let balthasar_history: Vec<APIMessage> = vec![];
    let casper_history: Vec<APIMessage> = vec![];

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![melchior, balthasar, casper])
        .manage(api_key.to_string())
        .manage(Mutex::new(MelchiorHistory(melchior_history)))
        .manage(Mutex::new(MelchCounter(0)))
        .manage(Mutex::new(BalthasarHistory(balthasar_history)))
        .manage(Mutex::new(BalthCounter(0)))
        .manage(Mutex::new(CasperHistory(casper_history)))
        .manage(Mutex::new(CasperCounter(0)))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
