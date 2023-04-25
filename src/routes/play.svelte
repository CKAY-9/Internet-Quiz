<script lang="ts">
	import { onMount} from "svelte";
    import { invoke } from "@tauri-apps/api";

    interface Question {
        prompt: string,
        answers: string[],
        correct_answer: number
    }

    // Page elements
    let countdown = document.getElementById("countdown");
    let pregame = document.getElementById("pregame");
    let game = document.getElementById("game");

	export let changeView: any;
	let timer = 0;
    let started = false;
    let question: Question = {
        "prompt": "",
        "answers": [],
        "correct_answer": 0
    };
    

    async function fetchQuestion() {
        question = JSON.parse(await invoke("get_question"));
    }

	onMount(async () => {
        await fetchQuestion();
        // Load elements
        countdown = document.getElementById("countdown");
        pregame = document.getElementById("pregame");
        game = document.getElementById("game");

		setInterval(async () => {
			if (timer > 3) {
                if (pregame !== null && game !== null && !started) {
                    pregame.style.transform = "translateX(-100vw)";
                    game.style.transform = "translateX(0)";
                    started = true;
                }
			} else {
				console.log(timer);
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
    <div class="questions">
        {#each question.answers as answer}
            <div class="question">{answer}</div>
	    {/each}
    </div>
</div>

<style>
    #game {
        transition: 1.5s ease transform;
        transform: translateX(100vw);

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-align: center;
    }

    #game .questions {
        display: grid;
        gap: 3em;
        grid-template-columns: 15rem 15rem;
        place-content: center;
    }
    #game .questions .question {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 2em 4em;
        background-color: rgb(var(--fg));
    }

    #pregame {
        top: 0;
        position: absolute;
        transition: 1.5s ease transform;
        text-align: center;
    }

	#countdown {
		font-size: 10em;
	}
</style>
