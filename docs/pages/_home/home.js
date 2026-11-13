// Pun homepage behaviour: the install copy button and the pun check run.
// The page makes no speed figures or comparisons; it says Pun is fast in words only.

const reduced = () => window.matchMedia("(prefers-reduced-motion: reduce)").matches;

function onView(el, fn, threshold = 0.35) {
  if (!el) return;
  if (!("IntersectionObserver" in window)) return fn();
  const io = new IntersectionObserver((entries) => {
    for (const e of entries) if (e.isIntersecting) { io.disconnect(); fn(); }
  }, { threshold });
  io.observe(el);
}

function installTabs(root) {
  for (const box of root.querySelectorAll("[data-install]")) {
    const text = box.querySelector("[data-command-text]");
    const copy = box.querySelector("[data-copy]");
    const tabs = [...box.querySelectorAll("[role=tab]")];
    for (const tab of tabs) {
      tab.addEventListener("click", () => {
        for (const t of tabs) t.setAttribute("aria-selected", t === tab ? "true" : "false");
        text.textContent = tab.dataset.command;
      });
    }
    copy?.addEventListener("click", async () => {
      try { await navigator.clipboard.writeText(text.textContent); } catch {}
      copy.textContent = "copied";
      setTimeout(() => (copy.textContent = "copy"), 1200);
    });
  }
}

// The output is visible by default. When the terminal scrolls into view, or on "Run again",
// the lines replay in quick succession: the run is over almost as soon as it starts.
function run(root) {
  const sec = root.querySelector("[data-run]");
  if (!sec) return;
  const play = () => {
    if (reduced()) return;
    sec.classList.remove("go");
    void sec.offsetWidth;
    sec.classList.add("go");
  };
  sec.querySelector("[data-run-replay]")?.addEventListener("click", play);
  onView(sec.querySelector(".term"), play, 0.5);
}

export function initHome(root = document) {
  const home = root.querySelector(".pun-home");
  if (!home || home.dataset.ready) return;
  home.dataset.ready = "1";
  installTabs(home);
  run(home);
}

initHome();
