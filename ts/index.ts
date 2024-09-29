const CANVAS_ELEMENT_ID = "game";

const loading = document.createElement("p");
loading.appendChild(document.createTextNode("loading..."));

document.body.appendChild(loading);

import("../pkg/index").then(async (wasm) => {
  const canvas = document.getElementById(
    CANVAS_ELEMENT_ID,
  ) as HTMLCanvasElement;
  console.log("done");
  document.body.removeChild(loading);

  const { clientHeight: parentHeight, clientWidth: parentWidth } =
    document.documentElement;
  if (parentHeight > parentWidth) {
    canvas.style.height = parentWidth + "px";
    canvas.style.width = parentWidth + "px";
  } else {
    canvas.style.height = parentHeight + "px";
    canvas.style.width = parentHeight + "px";
  }

  canvas.width = 2048;
  canvas.height = 2048;

  wasm.load_handlers(document.documentElement);
  await new Promise((resolve) => setTimeout(resolve, 1_000));

  // Initiate game loop
  let lastTime = Date.now();
  const startTime = lastTime;

  const render = () => {
    const now = Date.now();
    wasm.render(canvas as HTMLCanvasElement, now - lastTime, now - startTime);
    lastTime = now;
    requestAnimationFrame(render);
  };
  requestAnimationFrame(render);
});
