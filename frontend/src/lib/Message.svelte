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
</style>
