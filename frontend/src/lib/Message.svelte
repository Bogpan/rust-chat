<script lang="ts">
    import { onMount } from "svelte";

    export let kind: string;
    export let author: string | null;
    export let content: string;
    export let clientside: boolean;
    export let colour: string;
</script>

<div
    class="message-container"
    class:clientside
    class:server={kind === "server"}
>
    <div
        class="message-bubble"
        class:clientside
        class:server={kind === "server"}
    >
        {#if author !== null && !clientside}
            <div class="author" style="font-size: 12px; color: #{colour};">
                <p>{author}</p>
            </div>
        {/if}

        <div class="message">
            <p>
                {content}
            </p>
        </div>
    </div>
</div>

<style>
    .message-container {
        display: flex;
        margin-bottom: 1rem;
        font-size: 15px;
    }

    .message-container.clientside {
        justify-content: end;
    }

    .message-container.server {
        justify-content: center;
    }

    .message-bubble {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: start;
        width: max-content;
        background-color: #3a3a3a;
        border-radius: 10px;
        padding: 6px;
        transition: 200ms ease-in-out;
    }

    .message-bubble:hover {
        filter: brightness(115%);
    }

    .message-bubble.clientside {
        background-color: #e2852e;
    }

    .message-bubble.server {
        background-color: #535353;
    }

    p {
        margin: 0;
    }

    /* .author {
        color: #e2852e;
        font-size: 12px;
    } */

    /* .message {
        padding: 5px;
        display: flex;
        margin-bottom: 1rem;
        border-radius: 10px;
        background-color: #2b2b2b;
        transition: 100ms ease-in-out;
    }

    .clientside {
        justify-content: end;
    }

    .message.server {
        justify-content: center;
        background-color: #535353;
    }

    .message.server:hover {
        filter: brightness(115%);
    }

    .message p {
        margin: 0;
        background-color: #3a3a3a;
        border-radius: 10px;
        padding: 7px;
        font-size: 15px;
        transition: 100ms ease-in-out;
    }

    .message p.clientside {
        background-color: #e2852e;
    }

    .message p.server {
        background-color: #3a3a3a00;
    }

    .message p:hover {
        filter: brightness(115%);
        cursor: default;
    } */
</style>
