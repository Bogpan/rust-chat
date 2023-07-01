# rust-chat
A (bad) chat app using websockets, made with [`Axum`](https://github.com/tokio-rs/axum) and [`SvelteKit`](https://kit.svelte.dev/).

It uses SvelteKit to generate static files, served by the backend.

I made this project solely for the purpose of learning Axum and how websockets work and not a lot of thought was put into the frontend, but I think it's half-decent.
It is deployed to `shuttle.rs`: https://rust-chat.shuttleapp.rs/!
# Running
You need to have [Rust](https://www.rust-lang.org/tools/install), [npm](https://nodejs.org/en/download) and [cargo-shuttle](https://github.com/shuttle-hq/shuttle/releases/tag/v0.20.0) installed *and set up*.

In the `frontend` directory, run

```
npm run build
```

Which will create a `build` folder in the root.
Then, you can use

```
cargo shuttle project start
```
and
```
cargo shuttle run
```
