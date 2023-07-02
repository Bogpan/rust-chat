<script lang="ts">
    import Message from "$lib/Message.svelte";
    import { afterUpdate } from "svelte";

    let usernameInput: HTMLInputElement;
    let joinButton: HTMLButtonElement;
    let scrollSpan: HTMLSpanElement;

    let autoscroll = false;

    let username: string;
    let usernameValue = "";
    let input = "";
    let messages: {
        kind: string;
        author: string | null;
        content: string;
        clientside: boolean;
        colour: string;
    }[] = [];

    let colours = new Map();
    let websocket: WebSocket;

    async function joinChat() {
        if (joinButton.disabled) {
            return;
        }

        joinButton.disabled = true;
        usernameInput.disabled = true;

        websocket = new WebSocket(`wss://${window.location.host}/websocket`);
        // websocket = new WebSocket(`ws://localhost:3000/websocket`);

        websocket.addEventListener("open", (event) => {
            websocket.send(usernameValue);
            username = usernameValue;
        });

        websocket.addEventListener("message", (event) => {
            let message = JSON.parse(event.data);

            if (message.content == "Username already exists.") {
                joinButton.disabled = false;
                usernameInput.disabled = false;
            }

            if (message.author === username) {
                message.clientside = true;
            } else {
                message.clientside = false;
            }

            if (colours.has(message.author)) {
                message.colour = colours.get(message.author);
            } else {
                message.colour = generateRandomColour();
                colours.set(message.author, message.colour);
            }

            messages = [...messages, message];
            autoscroll = true;
        });

        websocket.addEventListener("error", (event) => {
            joinButton.disabled = false;
            // Failed to establish WebSocket connection to server.
            console.log(event);
            alert("Connection closed.");
        });
    }

    function generateRandomColour() {
        return Math.floor(Math.random() * 0xffffff)
            .toString(16)
            .padStart(6, "0");
    }

    afterUpdate(() => {
        if (autoscroll) {
            scrollSpan.scrollIntoView({ behavior: "smooth" });
            autoscroll = false;
        }
    });
</script>

<svelte:head>
    <title>Rust chat</title>
</svelte:head>

<h1>
    Chat app made with <a
        href="https://github.com/tokio-rs/axum"
        style="color: #21bad1;">Axum</a
    >
    and
    <a href="https://kit.svelte.dev/" style="color: #ff531a;">SvelteKit</a>.
</h1>

<div id="container">
    <div id="join-container">
        <input
            id="name-input"
            bind:value={usernameValue}
            bind:this={usernameInput}
            on:keydown={(e) => {
                if (
                    e.key == "Enter" &&
                    usernameValue !== "" &&
                    !joinButton.disabled
                ) {
                    joinChat();
                }
            }}
        />
        <button id="join-button" on:click={joinChat} bind:this={joinButton}
            >Join chat</button
        >
    </div>

    <div id="chat-container">
        <div id="chat-area">
            {#each messages as message}
                <Message {...message} />
            {/each}

            <span bind:this={scrollSpan} />
        </div>

        <input
            id="message-input"
            bind:value={input}
            on:keydown={(e) => {
                if (e.key == "Enter" && input !== "") {
                    websocket.send(input);
                    input = "";
                }
            }}
        />
    </div>
</div>

<style>
    @import url("https://fonts.googleapis.com/css2?family=Inter&display=swap");

    :global(body) {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        background-color: #212121;
        color: #dbdbdb;
        font-family: "Inter";
    }

    #container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    #join-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: max-content;
    }

    #chat-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        width: max-content;
    }

    input {
        background-color: #3a3a3a;
        /* border: 1px solid #131313; */
        border: none;
        color: white;
        padding: 7px;
        border-radius: 8px;
        width: 15rem;
        height: 1rem;
        margin: 8px;
    }

    input:focus {
        outline: none;
    }

    button {
        background-color: #3a3a3a;
        border: none;
        color: white;
        padding: 7px;
        border-radius: 8px;
        margin: 8px;
    }

    #chat-area {
        background-color: #2b2b2b;
        border: none;
        color: white;
        width: 110rem;
        height: 40rem;
        padding: 2rem;
        padding: 7px;
        border-radius: 8px;
        margin: 8px;
        resize: none;
        overflow: scroll;
    }

    #message-input {
        width: 110rem;
    }
</style>
