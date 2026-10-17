# Benchmark

The speed race. Bars are drawn to scale against the slowest row; competitors get the `stripe` pattern (`slow` tone), pun's own rows get `zap` (cold) and `gum` (warm). Values sit inside a long bar and beside a short one.

- Props: `rows` (`[{ label, sub, seconds, display?, tone: 'slow' | 'zap' | 'gum' | 'lilac' }]`), `title` (the project measured), `caption` (method), `href` (the "reproduce →" link).
- Every chart names the project, says lower is better and links to how to reproduce it. Only publish numbers the repo has measured (today: laravel/framework, Pint 190 s, pun 3.7 s cold, 0.07 s warm).
- Pair it with the `mega` type style for the multiple (51×).
