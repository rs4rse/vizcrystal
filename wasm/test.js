import * as vizmat from "./vizmat.js"; // <-- relative path to vizmat.js

async function main() {
    await vizmat.start();
    console.log("WASM loaded ✅");
}

main();

