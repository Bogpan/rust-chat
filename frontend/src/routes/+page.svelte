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

<div id="container">
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

<style>
    @import url("https://fonts.googleapis.com/css2?family=Inter&display=swap");

    :global(html, body) {
        width: 100%;
        height: 100%;
        margin: 0;
        padding: 0;
    }

    :global(body) {
        background-color: #212121;
        color: #dbdbdb;
        font-family: "Inter";
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    #container {
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
        flex: none;
        justify-content: center;
        align-items: center;
    }

    #name-input {
        margin-bottom: 1rem;
    }

    #chat-area {
        box-sizing: border-box;
        height: 100%;
        width: 95%;
        padding: 1rem;
        background-color: #2b2b2b;
        border-radius: 8px;
        overflow: scroll;
    }

    #message-input {
        box-sizing: border-box;
        width: 95%;
        margin-bottom: 1rem;
    }

    input {
        background-color: #3a3a3a;
        color: white;
        border: none;
        border-radius: 8px;
        padding: 7px;
        margin-top: 1rem;
    }

    input:focus {
        outline: none;
    }

    button {
        background-color: #3a3a3a;
        color: white;
        border: none;
        border-radius: 8px;
        padding: 7px;
        margin-bottom: 1rem;
    }

    button:hover {
        background-color: #4d4c4c;
    }

    button:active {
        background-color: #5e5c5c;
    }

    button:disabled {
        background-color: #2b2b2b;
    }
</style>
