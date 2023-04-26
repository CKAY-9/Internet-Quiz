<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api";

	interface Question {
		prompt: string;
		answers: string[];
		correct_answer: number;
	}

	// Page elements
	let countdown = document.getElementById("countdown");
	let pregame = document.getElementById("pregame");
	let game = document.getElementById("game");
	let end = document.getElementById("end");
	let nextBtn = document.getElementById("nextBtn");
	let questions = document.getElementById("questions");
	let success: HTMLAudioElement = document.getElementById("rightAudio") as any;
	let wrong: HTMLAudioElement = document.getElementById("wrongAudio") as any;

	export let changeView: any;
	let buttonLabel = ["A", "B", "C", "D"];
	let timer = 0;
	let started = false;
	let question: Question = {
		prompt: "",
		answers: [],
		correct_answer: 0
	};

	let score = 0;
	let currentQuestion = 0;
    let limit = 0;

	async function fetchQuestion() {
		question = JSON.parse(await invoke("get_question", { question: currentQuestion }));
	}

	onMount(async () => {
		// Fetch initial data
        limit = await invoke("question_limit");
		await fetchQuestion();

		// Load elements
		countdown = document.getElementById("countdown");
		pregame = document.getElementById("pregame");
		game = document.getElementById("game");
		end = document.getElementById("end");
		success = document.getElementById("rightAudio") as any;
		wrong = document.getElementById("wrongAudio") as any;
		nextBtn = document.getElementById("nextBtn");
		questions = document.getElementById("questions");

		success.volume = 0.25;
		wrong.volume = 1;

		setInterval(async () => {
			if (timer > 3) {
				if (pregame !== null && game !== null && !started) {
					pregame.style.transform = "translate(-100vw, -50%)";
					game.style.transform = "translateX(0)";
					started = true;
				}
			} else {
				if (countdown !== null) {
					countdown.textContent = `${3 - timer}`;
				}
			}
			timer++;
		}, 1000);
	});
</script>

<div id="pregame">
	<h1>Starting in:</h1>
	<h1 id="countdown">3</h1>
</div>

<div id="game">
	<h2>{question.prompt}</h2>
	<div class="questions" id="questions" style="margin-top: 2rem">
		{#each question.answers as answer, index}
			<button id={index.toString()}
				on:click={async () => {
					const selfBtn = document.getElementById(index.toString());
					if (index === question.correct_answer) {
						score++;
						if (success !== null) {
							success.play();
						}
						if (selfBtn !== null) {
							selfBtn.style.backgroundColor = "green";
						}
					} else {
						if (wrong !== null) {
							wrong.play();
						}
						if (selfBtn !== null) {
							selfBtn.style.backgroundColor = "red";
						}
					}
					currentQuestion++;
					if (currentQuestion >= limit) {
						if (end !== null && game !== null) {
							end.style.transform = "translate(0, -50%)";
							game.style.transform = "translateX(-100vw)";
						}
					} else {
						if (nextBtn !== null && questions !== null && selfBtn !== null) {
							questions.style.pointerEvents = "none";
							nextBtn.style.opacity = "1";
							nextBtn.style.pointerEvents = "auto";
						}	
					}
				}}
			class="question">
				<span style="font-weight: 900">{buttonLabel[index]}</span>
				<span>{answer}</span>
			</button>
		{/each}
	</div>
	<button id="nextBtn" style="opacity: 0; pointer-events: none; margin-top: 2rem" on:click={() => {
		if (game !== null && nextBtn !== null && questions !== null && success !== null && wrong !== null) {
			game.style.opacity = "0";
			nextBtn.style.opacity = "0";
			questions.style.pointerEvents = "auto";
			nextBtn.style.pointerEvents = "none";
			success.pause();
			success.currentTime = 0;
			wrong.pause();
			success.currentTime = 0;
		}

		for (let btnIndex = 0; btnIndex < question.answers.length; btnIndex++) {
			const currBtn = document.getElementById(btnIndex.toString());
			if (currBtn !== null) {
				currBtn.style.removeProperty("background-color");
			}
		}

		setTimeout(async () => {
			await fetchQuestion();
			if (game !== null) {
				game.style.opacity = "1";
			}
		}, 250);
	}}>Next Question</button>
</div>
<div id="end">
	<h1>You got {score} out of {limit}</h1>
	<button on:click={() => {
		if (end !== null) {
			end.style.opacity = "0";
		}
		changeView("home");
	}}>Home</button>
</div>

<audio src="/right.mp3" id="rightAudio"></audio>
<audio src="/wrong.mp3" id="wrongAudio"></audio>

<style>
	* {
		text-align: center;
	}

	#game {
		transition: 1.5s ease transform, 0.5s ease opacity;
		transform: translateX(100vw);

		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		width: 100%;
	}

	#nextBtn {
		transition: 0.5s ease all;
	}

	#end {
		top: 50%;
		transform: translate(200vw, -50%);
		position: absolute;
		transition: 1.5s ease transform, 0.5s ease opacity;
		
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		position: absolute;
	}

	#game .questions {
		display: grid;
		width: 100%;
		gap: 1em;
		place-content: center;
		grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr));
	}
	#game .questions .question {
		display: flex;
		gap: 1rem;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 1rem 1.5em;
		background-color: rgb(var(--fg));
		border: none;
		color: rgb(var(--text));
		border-radius: 0.5rem;
		font-size: 1em;
		transition: 0.5s ease all;
		box-shadow: none;
		text-transform: initial;
		font-weight: initial;
	}
	#game .questions .question:hover {
		transform: translateY(-5px);
		cursor: pointer;
		box-shadow: rgba(0, 0, 0, 0.25) 0px 14px 28px, rgba(0, 0, 0, 0.22) 0px 10px 10px;
		border-radius: 0.25rem;
	}

	#pregame {
		animation: 0.5s ease fadeIn;
		top: 50%;
		transform: translateY(-50%);
		position: absolute;
		transition: 1.5s ease transform;
		text-align: center;
	}

	#countdown {
		font-size: 10em;
	}
</style>
