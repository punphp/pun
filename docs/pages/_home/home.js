// Pun homepage behaviour: the install copy button, the terminal race, benchmark bars and count-ups.
// Every number on the page comes from BENCHMARKS. Add a group when a benchmark is measured; the tabs appear by themselves.

export const BENCHMARKS = [
  {
    id: "format",
    label: "Format",
    title: "pun format vs Pint 1.32.1",
    caption: "laravel/framework, whole repo, no cache, 8 cores",
    rows: [
      { name: "Pint", sub: "PHP", seconds: 294.7, kind: "other" },
      { name: "Pint --parallel", sub: "PHP, 8 processes", seconds: 41.4, kind: "other" },
      { name: "pun", sub: "Rust", seconds: 1.76, kind: "pun" },
    ],
  },
  // Next, as measured: { id: "lint", label: "Lint", title: "pun lint vs Rector", ... },
  // { id: "analyse", label: "Analyse", title: "pun analyse vs PHPStan + Larastan", ... }, Mago, Psalm.
];

const RACE = { project: "laravel/framework", files: 3158, slow: { name: "Pint", seconds: 294.7 }, fast: { name: "pun", seconds: 1.76 }, speed: 30 };

const reduced = () => window.matchMedia("(prefers-reduced-motion: reduce)").matches;
const fmt = (n, places = 1) => n.toLocaleString("en-US", { minimumFractionDigits: places, maximumFractionDigits: places });
const secs = (s) => (s < 1 ? `${s} s` : `${fmt(s, s % 1 ? 1 : 0)} s`);

function onView(el, fn, threshold = 0.35) {
  if (!el) return;
  if (!("IntersectionObserver" in window)) return fn();
  const io = new IntersectionObserver((entries) => {
    for (const e of entries) if (e.isIntersecting) { io.disconnect(); fn(); }
  }, { threshold });
  io.observe(el);
}

function countUp(el, to, ms = 1100) {
  const thousands = el.dataset.format === "thousands";
  const suffix = el.dataset.suffix || "";
  const show = (v) => (el.textContent = (thousands ? Math.round(v).toLocaleString("en-US") : Math.round(v)) + suffix);
  if (reduced()) return show(to);
  const t0 = performance.now();
  const tick = (t) => {
    const p = Math.min(1, (t - t0) / ms);
    show(to * (1 - Math.pow(1 - p, 3)));
    if (p < 1) requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
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

function race(root) {
  const sec = root.querySelector("[data-race]");
  if (!sec) return;
  const q = (s) => sec.querySelector(s);
  const lanes = {
    slow: { ...RACE.slow, clock: q('[data-clock="slow"]'), progress: q('[data-progress="slow"]'), result: q('[data-result="slow"]') },
    fast: { ...RACE.fast, clock: q('[data-clock="fast"]'), progress: q('[data-progress="fast"]'), result: q('[data-result="fast"]') },
  };
  const waiting = q("[data-waiting]");
  const DOTS = 64;
  let raf = 0;

  const finish = (lane) => {
    lane.clock.textContent = secs(lane.seconds);
    lane.progress.textContent = "·".repeat(DOTS);
    lane.result.innerHTML = `<span class="pass">PASS</span>${RACE.files.toLocaleString("en-US")} files`;
  };

  const run = () => {
    cancelAnimationFrame(raf);
    for (const lane of Object.values(lanes)) { lane.clock.textContent = "0.0 s"; lane.progress.textContent = ""; lane.result.textContent = ""; lane.done = false; }
    waiting.textContent = "";
    if (reduced()) { finish(lanes.slow); finish(lanes.fast); return; }
    const t0 = performance.now();
    const tick = (t) => {
      const real = Math.max(0, ((t - t0) / 1000) * RACE.speed); // seconds of real time this frame represents
      for (const lane of Object.values(lanes)) {
        if (lane.done) continue;
        if (real >= lane.seconds) { lane.done = true; finish(lane); continue; }
        lane.clock.textContent = secs(Math.max(0.1, Math.round(real * 10) / 10));
        lane.progress.textContent = "·".repeat(Math.floor((real / lane.seconds) * DOTS));
      }
      if (lanes.fast.done && !lanes.slow.done) {
        const ahead = Math.floor(real - lanes.fast.seconds);
        waiting.textContent = ahead > 0 ? `\n✓ done. pun has been waiting on Pint for ${ahead} s…` : "\n✓ done.";
      }
      if (lanes.slow.done) {
        waiting.textContent = `\n✓ done ${fmt(lanes.slow.seconds - lanes.fast.seconds)} s before Pint.`;
        return;
      }
      raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
  };

  q("[data-race-replay]")?.addEventListener("click", run);
  onView(sec, run, 0.4);
}

function bench(root) {
  const sec = root.querySelector("[data-bench]");
  if (!sec) return;
  const tabs = sec.querySelector("[data-bench-tabs]");
  const rowsEl = sec.querySelector("[data-bench-rows]");
  const title = sec.querySelector("[data-bench-title]");
  const caption = sec.querySelector("[data-bench-caption]");
  const mega = sec.querySelector("[data-mega]");
  const megaNote = sec.querySelector("[data-mega-note]");
  let seen = false;

  const render = (group) => {
    title.textContent = group.title;
    caption.textContent = group.caption;
    const max = Math.max(...group.rows.map((r) => r.seconds));
    rowsEl.innerHTML = group.rows
      .map((r) => `<div class="lane"><div class="who">${r.name}<small>${r.sub ?? ""}</small></div><div class="bar"><div class="fill ${r.kind}"></div><span class="val"></span></div></div>`)
      .join("");
    const lanes = [...rowsEl.querySelectorAll(".lane")];
    const grow = () => group.rows.forEach((r, i) => {
      const pct = r.seconds / max;
      const long = pct > 0.6;
      const fill = lanes[i].querySelector(".fill");
      const val = lanes[i].querySelector(".val");
      fill.style.width = long ? "100%" : `calc((100% - 90px) * ${pct.toFixed(5)} + 40px)`;
      val.classList.toggle("in", long);
      val.textContent = secs(r.seconds);
    });
    if (seen) requestAnimationFrame(() => requestAnimationFrame(grow));
    else onView(sec, () => { seen = true; requestAnimationFrame(grow); });

    const others = group.rows.filter((r) => r.kind === "other");
    const puns = group.rows.filter((r) => r.kind !== "other");
    if (others.length && puns.length) {
      // The headline compares with the first row, the upstream tool's default command.
      const x = Math.round(others[0].seconds / puns[0].seconds);
      const go = () => countUp(mega, x);
      seen ? go() : onView(sec, go);
      megaNote.textContent = `faster than ${others[0].name}.`;
    }
  };

  if (BENCHMARKS.length > 1) {
    tabs.innerHTML = BENCHMARKS.map((g, i) => `<button type="button" role="tab" aria-selected="${i === 0}" data-id="${g.id}">${g.label}</button>`).join("");
    tabs.addEventListener("click", (e) => {
      const b = e.target.closest("button");
      if (!b) return;
      for (const t of tabs.children) t.setAttribute("aria-selected", t === b ? "true" : "false");
      render(BENCHMARKS.find((g) => g.id === b.dataset.id));
    });
  }
  render(BENCHMARKS[0]);
}

export function initHome(root = document) {
  const home = root.querySelector(".pun-home");
  if (!home || home.dataset.ready) return;
  home.dataset.ready = "1";
  installTabs(home);
  race(home);
  bench(home);
  for (const el of home.querySelectorAll(".score [data-count-to], .burst [data-count-to]")) onView(el, () => countUp(el, Number(el.dataset.countTo)));
  // Cards stay visible by default; only those still below the fold get the drop-in entrance.
  if ("IntersectionObserver" in window && !reduced()) {
    for (const el of home.querySelectorAll(".stick")) {
      if (el.getBoundingClientRect().top < window.innerHeight) continue;
      el.classList.add("armed");
      onView(el, () => el.classList.add("in"), 0.2);
    }
  }
  onView(home.querySelector("[data-merge] .merge"), () => home.querySelector("[data-merge] .merge").classList.add("in"), 0.5);
}

initHome();
