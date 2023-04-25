#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use serde_json::Result;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize)]
struct Question {
	prompt: String,
	answers: Vec<String>,
	correct_answer: i32
}

#[tauri::command]
fn get_question() -> String {
	// TODO: Implement random shuffling of questions
	let questions = vec![
		Question {
			prompt: "What does TCP stand for?".to_string(),
			answers: vec!["Transmission Control Protocol".to_string(), "Tatical Cat Pistol".to_string(), "Terminate Childern Program".to_string(), "Transmission Command Protocol".to_string()],
			correct_answer: 0
		},
		Question {
			prompt: "What protocol is used for important information, such as banking?".to_string(),
			answers: vec!["TCP".to_string(), "UDP".to_string(), "ICUP".to_string(), "VOIP".to_string()],
			correct_answer: 0
		},
		Question {
			prompt: "Who owns the internet?".to_string(),
			answers: vec!["No one".to_string(), "Everyone".to_string(), "Both everyone and no one".to_string(), "I do".to_string()],
			correct_answer: 2
		}
	];

	let random_question = questions.choose(&mut rand::thread_rng()).unwrap();
	serde_json::to_string(&random_question).unwrap()
}

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![get_question])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
