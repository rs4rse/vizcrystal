# vizcrystal

Crystal visualizer go brrr

`vizcrystal` is an experimental project to build a crystal structure visualizer in Rust using [Bevy](https://bevyengine.org/).
The goal is to start with a desktop visualization prototype and later extend it to run in the browser via WebAssembly (WASM).

This project started as part of a study group session — going line by line through code, learning Bevy and Rust fundamentals together, and gradually building up toward a minimal viable product.

## Learning journey

This project is also an educational experiment:

* We discuss Rust basics, Bevy concepts, and code structure as a group (mix of rust beginners and some are already fluent in rust but new to bevy/wasm).
* Each session recaps progress, explains new code, and sets up the next steps
* By the end, several contributors will have hands-on knowledge of Bevy and Rust from scratch, a good foundation for an open-source project

## Project goals

Phase 1 – Desktop prototype

* Learn the basics of Bevy and ECS
* Render simple crystal structures on desktop
* Reach a small but working visualization demo

Phase 2 – WebAssembly (WASM)

* Port the visualization to run in browsers
* Understand Bevy’s WASM build pipeline
* Deliver a browser-based version of the crystal visualizer

Phase 3 – Extensibility

* Provide a foundation for further development (for example, interactivity, structure parsing, analysis)
* Build a shared base of knowledge for contributors (Rust, Bevy, WASM)

## Development setup

### Prerequisites

* Rust (latest stable)
* cargo (comes with Rust)
* For Bevy desktop builds: a working native toolchain (Linux, macOS, or Windows)
* For WASM builds (later):

  * wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`)
  * wasm-bindgen or trunk

### Running the project

```bash
# clone the repo
git clone https://github.com/rs4rse/vizcrystal.git
cd vizcrystal

# run the desktop version
cargo run
```

## Roadmap

* [x] Initial Bevy setup
* [ ] Basic crystal visualization (desktop)
* [ ] Browser support via WASM
* [ ] Extend with interactivity and file parsing
* [ ] Build community contributions

## Contributing

We are at an early stage and welcome contributions.
If you are new to Rust, Bevy, or visualization, this is the perfect playground.

## License

MIT

