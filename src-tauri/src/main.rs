#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Question {
	prompt: String,
	answers: Vec<String>,
	correct_answer: i32
}

fn questions() -> Vec<Question> {
	vec![
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
		},
		Question {
			prompt: "What is the internet?".to_string(),
			answers: vec!["A big fishing net".to_string(), "A network of computers connected together".to_string(), "A big word".to_string(), "I don't know...".to_string()],
			correct_answer: 1
		},
		Question {
			prompt: "How does the internet travel across the world?".to_string(),
			answers: vec!["People physically run to the destination".to_string(), "Power lines".to_string(), "Boats and Airplanes".to_string(), "Fiber-optic cables and waves".to_string()],
			correct_answer: 3
		},
		Question {
			prompt: "What is a DNS?".to_string(),
			answers: vec!["Dynamic Network Security".to_string(), "Associates IPs with domain names".to_string(), "A network security protocol".to_string(), "A website's backend server".to_string()],
			correct_answer: 1
		},
		Question {
			prompt: "Why is the internet's design good?".to_string(),
			answers: vec!["It is decentralized and distributed".to_string(), "It contains many security features and allows for user protection".to_string(), "It makes stuff look nice".to_string(), "It allows me to see stuff without going outside".to_string()],
			correct_answer: 0
		},
		Question {
			prompt: "How do you communicate with a website?".to_string(),
			answers: vec!["HTTP".to_string(), "Speaking the same human language".to_string(), "A mail service".to_string(), "Both A and B".to_string()],
			correct_answer: 0
		},
		Question {
			prompt: "What is an IP address?".to_string(),
			answers: vec!["A unique name for someone's cat".to_string(), "International postage address".to_string(), "A unique identifier for each computer connected to a network".to_string(), "Something made up by dogs to take over the world".to_string()],
			correct_answer: 2
		},
		Question {
			prompt: "When was the internet made?".to_string(),
			answers: vec!["1983".to_string(), "2002".to_string(), "1823".to_string(), "1634".to_string()],
			correct_answer: 0
		}
	]
}

#[tauri::command]
fn get_question(question: usize) -> String {
	let random_question = &questions()[question];
	serde_json::to_string(&random_question).unwrap()
}

#[tauri::command]
fn question_limit() -> usize {
	questions().len()
}

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![get_question, question_limit])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
