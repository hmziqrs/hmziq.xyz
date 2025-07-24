console.log("Hello World!");

const cursorId = "cursor";
const cursorTrailId = "cursor-trail";

let mouse = { x: 0, y: 0 };

let dom = {
  cursor: null,
  cursorTrail: null,
};

function main() {
  dom.cursor = document.getElementById(cursorId);
  dom.cursorTrail = document.getElementById(cursorTrailId);
}

function cursorMove() {
  console.log(mouse);
  dom.cursor.style.transform = `translate(${mouse.x}px, ${mouse.y}px)`;
  dom.cursorTrail.style.transform = `translate(${mouse.x}px, ${mouse.y}px)`;
}

window.addEventListener("mousemove", (event) => {
  mouse.x = event.clientX;
  mouse.y = event.clientY;

  cursorMove();
});

let duration = 50;
let timeout = null;

function check() {
  if (document.getElementById(cursorId)) {
    if (!!timeout) {
      clearTimeout(timeout);
      timeout = null;
    }
    main();
  } else {
    timeout = setTimeout(check, Math.min(200, duration));
    duration *= 2;
  }
}

check();
