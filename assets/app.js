console.log("Hello World!");

const cursorId = "cursor";
const cursorTrailId = "cursor-trail";
const coordinatesId = "coordinates";

let mouse = { x: 0, y: 0 };
let scrollY = 0;
let isOverClickable = false;

let dom = {
  cursor: null,
  cursorTrail: null,
  coordinates: null,
};
let custom = {
  cursor: null,
};

function main() {
  dom.cursor = document.getElementById(cursorId);
  dom.cursorTrail = document.getElementById(cursorTrailId);
  dom.coordinates = document.getElementById(coordinatesId);
}

function detectClickableElement(event) {
  const target = event.target;

  // Check if element is an anchor tag or has .btn class
  const isClickable =
    target.tagName === "A" || target.classList.contains("btn");

  // Also check parent elements in case of nested structure
  let parent = target.parentElement;
  let foundClickable = isClickable;

  while (parent && !foundClickable) {
    foundClickable = parent.tagName === "A" || parent.classList.contains("btn");
    parent = parent.parentElement;
  }

  const wasOverClickable = isOverClickable;
  isOverClickable = foundClickable;

  // Log state changes for debugging
  if (wasOverClickable !== isOverClickable) {
    console.log(
      `Cursor ${isOverClickable ? "entered" : "left"} clickable element`,
    );
    updateCursor();
  }

  return isOverClickable;
}

function calculateCoordinates() {
  const windowWidth = window.innerWidth;
  const windowHeight = window.innerHeight;

  const lat = (mouse.y / windowHeight) * 180.0 - 90.0;
  const lng = (mouse.x / windowWidth) * 360.0 - 180.0;

  const latDir = lat > 0 ? "N" : "S";
  const lngDir = lng > 0 ? "E" : "W";

  return `${Math.abs(lat).toFixed(3)}° ${latDir}, ${Math.abs(lng).toFixed(3)}° ${lngDir}`;
}

function updateCoordinates() {
  if (dom.coordinates) {
    const coordText = calculateCoordinates();
    const coordSpan = dom.coordinates.querySelector(".coord-text");
    if (coordSpan) {
      coordSpan.textContent = coordText;
    }

    // Update transform for scroll offset
    dom.coordinates.style.transform = `translate(0, ${scrollY}px)`;
  }
}

function updateCursor() {
  if (dom.cursor) {
    let scale = 1;
    if () {
//
    }
    else if (isOverClickable) {
      scale = 2.2;
    }
    dom.cursor.style.transform = `translate(${mouse.x}px, ${mouse.y}px) scale(${scale})`;
  }
}

function cursorMove() {
  if (dom.cursorTrail) {
    dom.cursorTrail.style.transform = `translate(${mouse.x}px, ${mouse.y}px) scale(1)`;
  }
}

window.addEventListener("mousemove", (event) => {
  mouse.x = event.clientX;
  mouse.y = event.clientY;
  updateCursor();
  cursorMove();
  updateCoordinates();
  detectClickableElement(event);
});

window.addEventListener("scroll", () => {
  scrollY = window.pageYOffset;
  updateCoordinates();
});

window.addEventListener("resize", () => {
  updateCoordinates();
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

// Expose functions to window for Rust/WASM access
window.main = main;
window.isOverClickable = () => isOverClickable;
