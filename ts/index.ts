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

  // Initiate game loop
  let lastTime = Date.now();
  const startTime = lastTime;

  const render = () => {
    const now = Date.now();
    const won = wasm.render(
      canvas as HTMLCanvasElement,
      now - lastTime,
      now - startTime,
    );
    if (won) {
      const youWonTextNode = document.createTextNode("Happy 23rd!");
      const pNode = document.createElement("p");
      pNode.style.color = "white";
      pNode.style.fontSize = "50px";
      pNode.style.fontFamily = "Arial";
      pNode.appendChild(youWonTextNode);

      canvas.parentElement.appendChild(pNode);

      canvas.remove();
    } else {
      lastTime = now;
      requestAnimationFrame(render);
    }
  };
  requestAnimationFrame(render);
});
