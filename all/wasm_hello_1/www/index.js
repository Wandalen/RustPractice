// import * as wasm from "hello-wasm-pack";

// wasm.greet();

import("../pkg").then(module => {
  module.run();
});
