const CANVAS_ELEMENT_ID = "game";

const loading = document.createElement("p");
loading.appendChild(document.createTextNode("loading..."));

document.body.appendChild(loading);

import("../pkg/index").then((wasm) => {
  const canvas = document.getElementById(CANVAS_ELEMENT_ID);
  console.log("done");
  document.body.removeChild(loading);
  wasm.render(canvas as HTMLCanvasElement, 0);
});
