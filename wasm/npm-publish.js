import * as vizmat from "./vizmat.js";

async function main() {
    await vizmat.start();
    console.log("WASM loaded ✅");
}

main();

