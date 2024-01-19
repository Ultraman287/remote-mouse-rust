const drag_zone = document.getElementById("drag_zone");

// At the initial touch of the mouse button/finger, the dragstart event is fired where the location of the pointer is stored and the delta is calculated from each previous position and
// sent to the server via an http request as a JSON object.

console.log(drag_zone);

let delta = { x: 0, y: 0 };
let clientX = 0;
let clientY = 0;

let drag = false;

drag_zone.addEventListener("mousedown", (e) => {
  e.preventDefault();
  e.stopPropagation();
  clientX = e.clientX;
  clientY = e.clientY;
  const delta = { x: 0, y: 0 };
  console.log("mousedown");
  drag = true;
});

drag_zone.addEventListener("touchstart", (e) => {
  e.preventDefault();
  e.stopPropagation();
  clientX = e.touches[0].clientX;
  clientY = e.touches[0].clientY;
  const delta = { x: 0, y: 0 };
  console.log("touchstart");
  drag = true;
});

// At each subsequent movement of the mouse/finger, the delta is calculated and sent to the server via an http request as a JSON object.

drag_zone.addEventListener("mousemove", (e) => {
  if (drag) {
    e.preventDefault();
    e.stopPropagation();
    delta.x = e.clientX - clientX;
    delta.y = e.clientY - clientY;
    clientX = e.clientX;
    clientY = e.clientY;
    console.log("mousemove");
    fetch("/move_mouse", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(delta),
    });
  }
});

drag_zone.addEventListener("touchmove", (e) => {
  if (drag) {
    e.preventDefault();
    e.stopPropagation();
    delta.x = e.touches[0].clientX - clientX;
    delta.y = e.touches[0].clientY - clientY;
    // Converting the delta to an integer to avoid sending floats to the server.
    delta.x = Math.round(delta.x);
    delta.y = Math.round(delta.y);
    clientX = e.touches[0].clientX;
    clientY = e.touches[0].clientY;
    console.log("touchmove");
    fetch("/move_mouse", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(delta),
    });
  }
});

// When the mouse button/finger is released, the drag variable is set to false and the delta is reset to zero.

drag_zone.addEventListener("mouseup", (e) => {
  e.preventDefault();
  e.stopPropagation();
  drag = false;
  delta.x = 0;
  delta.y = 0;
  console.log("mouseup");
});

drag_zone.addEventListener("touchend", (e) => {
  e.preventDefault();
  e.stopPropagation();
  drag = false;
  delta.x = 0;
  delta.y = 0;
  console.log("touchend");
});

// As soon as the pointer leaves the drag zone, the drag variable is set to false and the delta is reset to zero.

drag_zone.addEventListener("mouseleave", (e) => {
  e.preventDefault();
  e.stopPropagation();
  drag = false;
  delta.x = 0;
  delta.y = 0;
  console.log("mouseleave");
});

drag_zone.addEventListener("touchcancel", (e) => {
  e.preventDefault();
  e.stopPropagation();
  drag = false;
  delta.x = 0;
  delta.y = 0;
  console.log("touchcancel");
});
