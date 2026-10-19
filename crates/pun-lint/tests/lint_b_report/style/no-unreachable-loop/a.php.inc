<?php

for (;;) { break; }
for (;;): return; endfor;
foreach ($a as $b) break 1;
foreach ($a as $b): { break; } endforeach;
while (true): return 1; endwhile;
while (true) { break 2; }
do { break; } while (true);
do { if ($x) { continue; } break; } while (true);
while (true) { if ($x) continue; else return; }
while (true) { if ($x) {} elseif ($y) { goto end; } break; }
while (true) { if ($x) {} else { continue; } break; }
while (true) { if ($x): continue; endif; break; }
while (true) { if ($x): elseif ($y): continue; endif; break; }
while (true) { if ($x): else: continue; endif; break; }
while (true) { if ($x) {} break; }
while (true) { while ($y) { continue; } break; }
while (true) { while ($y): continue; endwhile; break; }
while (true) { do { continue; } while ($y); break; }
while (true) { for (;;) continue; break; }
while (true) { for (;;): continue; endfor; break; }
while (true) { foreach ($a as $b) continue; break; }
while (true) { foreach ($a as $b): continue; endforeach; break; }
while (true) { declare(ticks=1) { continue; } break; }
while (true) { declare(ticks=1): continue; enddeclare; break; }
while (true) { declare(ticks=1); break; }
while (true) { try { continue; } finally {} break; }
while (true) { try {} catch (E $e) { continue; } break; }
while (true) { try {} finally { continue; } break; }
while (true) { try {} catch (E $e) {} break; }
while (true) { switch ($x) { case 1: continue 2; } break; }
while (true) { switch ($x) { case 1: echo 1; } break; }
while (true) { { continue; } break; }
while (true) { foo(); }
while (true) { foo(); break; }
while (true) { break $x; }
while (true) { break 0x1; }
while (true) { { { return; } } }
end:
-----
3:1: Loop is unconditionally terminated and will not iterate.
4:1: Loop is unconditionally terminated and will not iterate.
5:1: Loop is unconditionally terminated and will not iterate.
6:1: Loop is unconditionally terminated and will not iterate.
7:1: Loop is unconditionally terminated and will not iterate.
9:1: Loop is unconditionally terminated and will not iterate.
17:1: Loop is unconditionally terminated and will not iterate.
27:1: Loop is unconditionally terminated and will not iterate.
31:1: Loop is unconditionally terminated and will not iterate.
33:1: Loop is unconditionally terminated and will not iterate.
36:1: Loop is unconditionally terminated and will not iterate.
38:1: Loop is unconditionally terminated and will not iterate.
39:1: Loop is unconditionally terminated and will not iterate.
