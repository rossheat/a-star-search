#  A* Search

![a star search](./astar.png)

- An implementation of the A* search algorithm.
- Uses Euclidean distance as the heuristic function.
- Obstacles, start and goal points are placed randomly.
- Demo: https://rossheat.github.io/a-star-search/

## Launch

Install Rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add the Wasm target:

```
rustup target add wasm32-unknown-unknown
```

Install wasm-pack:

```
cargo install wasm-pack
```

Start the server:

```
npx live-server
```

The app should open in your browser.

## Development

Ensure you have nodemon installed:

```
npm i -g nodemon
```

Start the nodemon file watcher:

```
nodemon
```

Then launch the app (see above).
