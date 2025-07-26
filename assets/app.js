console.log("Hello World!");

const cursorId = "cursor";
const cursorTrailId = "cursor-trail";
const coordinatesId = "coordinates";

let mouse = { x: 0, y: 0 };
let scrollY = 0;
let isOverClickable = false;
let isMouseDown = false;

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
    if (isMouseDown) {
      scale = 0.5;
    } else if (isOverClickable) {
      scale = 2.2;
    }
    dom.cursor.style.transform = `translate(${mouse.x}px, ${mouse.y}px) scale(${scale})`;
  }
}

function updateCursorTrail() {
  if (dom.cursorTrail) {
    let x = mouse.x;
    let y = mouse.y;
    if (isOverClickable || isMouseDown) {
      x += 7;
      y += 7;
    }
    dom.cursorTrail.style.transform = `translate(${x}px, ${y}px) scale(1)`;
  }
}

window.addEventListener("mousemove", (event) => {
  mouse.x = event.clientX;
  mouse.y = event.clientY;
  updateCursor();
  updateCursorTrail();
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

window.addEventListener("mousedown", () => {
  isMouseDown = true;
  updateCursor();
  updateCursorTrail();
});

window.addEventListener("mouseup", () => {
  isMouseDown = false;
  updateCursor();
  updateCursorTrail();
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

// Ather Animation System
const atherCanvasId = "ather-canvas";
const atherWrapperId = "ather-wrapper";

// Utility functions from ather.js
const { cos, sin, PI, floor, min, max, abs, sqrt } = Math;
const TAU = PI * 2;
const rand = (max) => Math.random() * max;
const randIn = (min, max) => Math.random() * (max - min) + min;
const fadeInOut = (t, m) => {
  const hm = 0.5 * m;
  return abs(((t + hm) % m) - hm) / hm;
};
const lerp = (a, b, t) => a + (b - a) * t;

function createRenderingContext() {
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");
  const buffer = document.createElement("canvas");
  const bufferCtx = buffer.getContext("2d");

  return { ctx, buffer: bufferCtx };
}

// SimplexNoise implementation
class SimplexNoise {
  constructor() {
    this.grad3 = [
      [1, 1, 0],
      [-1, 1, 0],
      [1, -1, 0],
      [-1, -1, 0],
      [1, 0, 1],
      [-1, 0, 1],
      [1, 0, -1],
      [-1, 0, -1],
      [0, 1, 1],
      [0, -1, 1],
      [0, 1, -1],
      [0, -1, -1],
    ];
    this.p = [];
    for (let i = 0; i < 256; i++) {
      this.p[i] = floor(Math.random() * 256);
    }
    this.perm = [];
    for (let i = 0; i < 512; i++) {
      this.perm[i] = this.p[i & 255];
    }
  }

  dot(g, x, y, z) {
    return g[0] * x + g[1] * y + g[2] * z;
  }

  noise3D(xin, yin, zin) {
    const F3 = 1.0 / 3.0;
    const G3 = 1.0 / 6.0;

    const s = (xin + yin + zin) * F3;
    const i = floor(xin + s);
    const j = floor(yin + s);
    const k = floor(zin + s);
    const t = (i + j + k) * G3;
    const X0 = i - t;
    const Y0 = j - t;
    const Z0 = k - t;
    const x0 = xin - X0;
    const y0 = yin - Y0;
    const z0 = zin - Z0;

    let i1, j1, k1;
    let i2, j2, k2;

    if (x0 >= y0) {
      if (y0 >= z0) {
        i1 = 1;
        j1 = 0;
        k1 = 0;
        i2 = 1;
        j2 = 1;
        k2 = 0;
      } else if (x0 >= z0) {
        i1 = 1;
        j1 = 0;
        k1 = 0;
        i2 = 1;
        j2 = 0;
        k2 = 1;
      } else {
        i1 = 0;
        j1 = 0;
        k1 = 1;
        i2 = 1;
        j2 = 0;
        k2 = 1;
      }
    } else {
      if (y0 < z0) {
        i1 = 0;
        j1 = 0;
        k1 = 1;
        i2 = 0;
        j2 = 1;
        k2 = 1;
      } else if (x0 < z0) {
        i1 = 0;
        j1 = 1;
        k1 = 0;
        i2 = 0;
        j2 = 1;
        k2 = 1;
      } else {
        i1 = 0;
        j1 = 1;
        k1 = 0;
        i2 = 1;
        j2 = 1;
        k2 = 0;
      }
    }

    const x1 = x0 - i1 + G3;
    const y1 = y0 - j1 + G3;
    const z1 = z0 - k1 + G3;
    const x2 = x0 - i2 + 2.0 * G3;
    const y2 = y0 - j2 + 2.0 * G3;
    const z2 = z0 - k2 + 2.0 * G3;
    const x3 = x0 - 1.0 + 3.0 * G3;
    const y3 = y0 - 1.0 + 3.0 * G3;
    const z3 = z0 - 1.0 + 3.0 * G3;

    const ii = i & 255;
    const jj = j & 255;
    const kk = k & 255;
    const gi0 = this.perm[ii + this.perm[jj + this.perm[kk]]] % 12;
    const gi1 =
      this.perm[ii + i1 + this.perm[jj + j1 + this.perm[kk + k1]]] % 12;
    const gi2 =
      this.perm[ii + i2 + this.perm[jj + j2 + this.perm[kk + k2]]] % 12;
    const gi3 = this.perm[ii + 1 + this.perm[jj + 1 + this.perm[kk + 1]]] % 12;

    let n0, n1, n2, n3;
    let t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0;
    if (t0 < 0) n0 = 0.0;
    else {
      t0 *= t0;
      n0 = t0 * t0 * this.dot(this.grad3[gi0], x0, y0, z0);
    }

    let t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1;
    if (t1 < 0) n1 = 0.0;
    else {
      t1 *= t1;
      n1 = t1 * t1 * this.dot(this.grad3[gi1], x1, y1, z1);
    }

    let t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2;
    if (t2 < 0) n2 = 0.0;
    else {
      t2 *= t2;
      n2 = t2 * t2 * this.dot(this.grad3[gi2], x2, y2, z2);
    }

    let t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3;
    if (t3 < 0) n3 = 0.0;
    else {
      t3 *= t3;
      n3 = t3 * t3 * this.dot(this.grad3[gi3], x3, y3, z3);
    }

    return 32.0 * (n0 + n1 + n2 + n3);
  }
}

// Ather animation state
let ather = {
  canvas: null,
  wrapper: null,
  ctx: null,
  buffer: null,
  bufferCtx: null,
  particleCount: 2000,
  currentParticleCount: 0,
  particlePropCount: 9,
  particlePropsLength: 0,
  particleProps: null,
  center: [0, 0],
  tick: 0,
  simplex: null,
  animationId: null,
  isAnimating: false,
  hasFormed: false,
  observer: null,
  formationStartTime: 0,
  formationDuration: 3000, // 3 seconds to form all particles
};

function createAtherRenderingContext() {
  ather.canvas = document.getElementById(atherCanvasId);
  ather.wrapper = document.getElementById(atherWrapperId);

  if (!ather.canvas) return false;

  ather.ctx = ather.canvas.getContext("2d");

  // Create buffer canvas
  ather.buffer = document.createElement("canvas");
  ather.bufferCtx = ather.buffer.getContext("2d");

  return true;
}

function initAtherParticle(i) {
  const spawnRadius = rand(150) + 150;
  const rd = rand(spawnRadius);
  const rt = rand(TAU);
  const cx = cos(rt);
  const sy = sin(rt);
  const x = ather.center[0] + cx * rd;
  const y = ather.center[1] + sy * rd;
  const rv = randIn(0.1, 1);
  const s = randIn(1, 8);
  const vx = rv * cx * 0.1;
  const vy = rv * sy * 0.1;
  const w = randIn(0.1, 2);
  const h = randIn(160, 260);
  const l = 0;
  const ttl = randIn(50, 200);

  ather.particleProps.set([x, y, vx, vy, s, h, w, l, ttl], i);
}

function drawAtherParticle(i) {
  const props = new Float32Array(ather.particlePropCount);
  for (let j = 0; j < ather.particlePropCount; j++) {
    props[j] = ather.particleProps[i + j];
  }

  let [x, y, vx, vy, s, h, w, l, ttl] = props;

  const noiseSteps = 6;
  const n =
    ather.simplex.noise3D(x * 0.0025, y * 0.0025, ather.tick * 0.0005) *
    TAU *
    noiseSteps;
  vx = lerp(vx, cos(n), 0.05);
  vy = lerp(vy, sin(n), 0.05);
  const dx = x + vx * s;
  const dy = y + vy * s;
  const dl = fadeInOut(l, ttl);
  const c = `hsla(${h},50%,60%,${dl})`;

  l++;

  ather.bufferCtx.save();
  ather.bufferCtx.lineWidth = dl * w + 1;
  ather.bufferCtx.strokeStyle = c;
  ather.bufferCtx.beginPath();
  ather.bufferCtx.moveTo(x, y);
  ather.bufferCtx.lineTo(dx, dy);
  ather.bufferCtx.stroke();
  ather.bufferCtx.closePath();
  ather.bufferCtx.restore();

  ather.particleProps.set([dx, dy, vx, vy, s, h, w, l, ttl], i);

  const outOfBounds =
    dx > ather.buffer.width || dx < 0 || dy > ather.buffer.height || dy < 0;
  if (outOfBounds || l > ttl) {
    initAtherParticle(i);
  }
}

function resizeAther() {
  if (!ather.canvas) return;

  const section = document.getElementById("contact");
  if (!section) return;

  const rect = section.getBoundingClientRect();
  ather.canvas.width = section.offsetWidth;
  ather.canvas.height = section.offsetHeight;

  ather.buffer.width = ather.canvas.width;
  ather.buffer.height = ather.canvas.height;

  ather.center[0] = ather.canvas.width * 0.5;
  ather.center[1] = ather.canvas.height * 0.5;
}

function drawAther() {
  if (!ather.isAnimating) return;

  ather.tick++;

  // Calculate how many particles should be active based on formation time
  if (!ather.hasFormed) {
    const elapsed = Date.now() - ather.formationStartTime;
    const progress = Math.min(elapsed / ather.formationDuration, 1);
    // Ease-in-out curve for smooth formation
    const eased =
      progress < 0.5
        ? 2 * progress * progress
        : 1 - Math.pow(-2 * progress + 2, 2) / 2;
    ather.currentParticleCount = Math.floor(ather.particleCount * eased);

    if (progress >= 1) {
      ather.hasFormed = true;
      ather.currentParticleCount = ather.particleCount;
    }
  }

  ather.bufferCtx.clearRect(0, 0, ather.buffer.width, ather.buffer.height);

  ather.ctx.fillStyle = "rgba(0,0,0,0.1)";
  ather.ctx.fillRect(0, 0, ather.canvas.width, ather.canvas.height);

  // Only draw active particles
  const activeParticles = ather.currentParticleCount * ather.particlePropCount;
  for (let i = 0; i < activeParticles; i += ather.particlePropCount) {
    drawAtherParticle(i);
  }

  // Apply effects
  ather.ctx.save();
  ather.ctx.filter = "blur(8px)";
  ather.ctx.globalCompositeOperation = "lighten";
  ather.ctx.drawImage(ather.buffer, 0, 0);
  ather.ctx.restore();

  ather.ctx.save();
  ather.ctx.globalCompositeOperation = "lighter";
  ather.ctx.drawImage(ather.buffer, 0, 0);
  ather.ctx.restore();

  ather.animationId = window.requestAnimationFrame(drawAther);
}

function startAtherAnimation() {
  if (ather.isAnimating) return;

  ather.isAnimating = true;
  if (!ather.hasFormed) {
    ather.formationStartTime = Date.now();
  }

  // Add opacity class for fade in effect
  if (ather.wrapper) {
    ather.wrapper.classList.add("opacity-100");
  }

  drawAther();
}

function stopAtherAnimation() {
  ather.isAnimating = false;

  // Remove opacity class for fade out effect
  if (ather.wrapper) {
    ather.wrapper.classList.remove("opacity-100");
  }

  // Continue rendering for a bit to allow fade out to complete
  setTimeout(() => {
    if (!ather.isAnimating && ather.animationId) {
      window.cancelAnimationFrame(ather.animationId);
      ather.animationId = null;
    }
  }, 1000); // Match the CSS transition duration
}

function setupAther() {
  if (!createAtherRenderingContext()) return;

  ather.simplex = new SimplexNoise();
  ather.particlePropsLength = ather.particleCount * ather.particlePropCount;
  ather.particleProps = new Float32Array(ather.particlePropsLength);

  resizeAther();

  // Initialize all particles
  for (let i = 0; i < ather.particlePropsLength; i += ather.particlePropCount) {
    initAtherParticle(i);
  }

  // Set up intersection observer
  const options = {
    root: null,
    rootMargin: "0px",
    threshold: 0.5, // Trigger when 50% visible
  };

  ather.observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        startAtherAnimation();
      } else {
        stopAtherAnimation();
      }
    });
  }, options);

  const contactSection = document.getElementById("contact");
  if (contactSection) {
    ather.observer.observe(contactSection);
  }
}

// Check for ather canvas and initialize
function checkAther() {
  if (document.getElementById(atherCanvasId)) {
    setupAther();
  } else {
    setTimeout(checkAther, 100);
  }
}

checkAther();

// Handle resize
window.addEventListener("resize", () => {
  if (ather.canvas) {
    resizeAther();
  }
});
