console.log("Hello World!");

const cursorId = "cursor";
const cursorTrailId = "cursor-trail";
const coordinatesId = "coordinates";

let mouse = { x: 0, y: 0 };
let scrollY = 0;

let dom = {
  cursor: null,
  cursorTrail: null,
  coordinates: null,
};

function main() {
  dom.cursor = document.getElementById(cursorId);
  dom.cursorTrail = document.getElementById(cursorTrailId);
  dom.coordinates = document.getElementById(coordinatesId);
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
    const coordSpan = dom.coordinates.querySelector('.coord-text');
    if (coordSpan) {
      coordSpan.textContent = coordText;
    }
    
    // Update transform for scroll offset
    dom.coordinates.style.transform = `translate(0, ${scrollY}px)`;
  }
}

function cursorMove() {
  if (dom.cursor) {
    dom.cursor.style.transform = `translate(${mouse.x}px, ${mouse.y}px)`;
  }
  if (dom.cursorTrail) {
    dom.cursorTrail.style.transform = `translate(${mouse.x}px, ${mouse.y}px)`;
  }
  updateCoordinates();
}

window.addEventListener("mousemove", (event) => {
  mouse.x = event.clientX;
  mouse.y = event.clientY;
  cursorMove();
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
