<?php

foreach ($a as $b) {
    if ($b) {
        foo($b);
        bar($b);
    }
}
foreach ($a as $b) {
    if (!$b->ok()) {
        foo($b);
    }
}
foreach ($a as $b) {
    if ($b == 1) { foo(); }
}
foreach ($a as $b) { if ($b != 1) { foo(); } }
foreach ($a as $b) { if ($b === 1) { foo(); } }
foreach ($a as $b) { if ($b !== 1) { foo(); } }
foreach ($a as $b) { if ($b <> 1) { foo(); } }
foreach ($a as $b) { if ($b < 1) { foo(); } }
foreach ($a as $b) { if ($b <= 1) { foo(); } }
foreach ($a as $b) { if ($b > 1) { foo(); } }
foreach ($a as $b) { if ($b >= 1) { foo(); } }
foreach ($a as $b) { if ($b && $c) { foo(); } }
foreach ($a as $b) { if (-$b) { foo(); } }
foreach ($a as $b) if ($b) foo();
foreach ($a as $b) if ($b) /* c */ foo();
foreach ($a as $b) { if ($b) { /* c */ foo(); /* d */ } }
foreach ($a as $b): if ($b): foo(); endif; endforeach;
foreach ($a as $b): if ($b): /* c */ foo(); /* d */ endif; endforeach;
foreach ($a as $b) { if ($b) { foo(); } else { bar(); } }
foreach ($a as $b) { if ($b) { foo(); } elseif ($c) { bar(); } }
foreach ($a as $b): if ($b): foo(); else: bar(); endif; endforeach;
foreach ($a as $b): if ($b): foo(); elseif ($c): bar(); endif; endforeach;
foreach ($a as $b) { if ($b) { continue; } }
foreach ($a as $b) { if ($b) break; }
foreach ($a as $b) { if ($b) { return; } }
foreach ($a as $b) { if ($b) { throw new E(); } }
foreach ($a as $b) { if ($b) { { continue; } } }
foreach ($a as $b) { if ($b) { { continue; foo(); } } }
foreach ($a as $b): if ($b): continue; endif; endforeach;
foreach ($a as $b) { if ($b) { } }
foreach ($a as $b) { if ($b): endif; }
foreach ($a as $b) { if ($b) {} ; }
foreach ($a as $b) { ; if ($b) { foo(); } ; }
foreach ($a as $b) { foo(); if ($b) { foo(); } }
foreach ($a as $b) { { if ($b) { foo(); } } }
foreach ($a as $b) { {} if ($b) { foo(); } }
foreach ($a as $b) { if ($b) { foo(); } $x; }
foreach ($a as $b) { if ($b) { echo 1; } }
for (;;) { if ($b) { foo(); } }
for (;;): if ($b) { foo(); } endfor;
while (1) { if ($b) { foo(); } }
while (1): if ($b) { foo(); } endwhile;
do { if ($b) { foo(); } } while (1);
do if ($b) { foo(); } while (1);
foreach ($a as $b) foo();
foreach ($a as $b) { if ($b) { foo(); } if ($c) { bar(); } }
-----
4:5: Consider using early continue pattern to reduce nesting.
10:5: Consider using early continue pattern to reduce nesting.
15:5: Consider using early continue pattern to reduce nesting.
17:22: Consider using early continue pattern to reduce nesting.
18:22: Consider using early continue pattern to reduce nesting.
19:22: Consider using early continue pattern to reduce nesting.
20:22: Consider using early continue pattern to reduce nesting.
21:22: Consider using early continue pattern to reduce nesting.
22:22: Consider using early continue pattern to reduce nesting.
23:22: Consider using early continue pattern to reduce nesting.
24:22: Consider using early continue pattern to reduce nesting.
25:22: Consider using early continue pattern to reduce nesting.
26:22: Consider using early continue pattern to reduce nesting.
27:20: Consider using early continue pattern to reduce nesting.
28:20: Consider using early continue pattern to reduce nesting.
29:22: Consider using early continue pattern to reduce nesting.
30:21: Consider using early continue pattern to reduce nesting.
31:21: Consider using early continue pattern to reduce nesting.
41:22: Consider using early continue pattern to reduce nesting.
46:24: Consider using early continue pattern to reduce nesting.
48:24: Consider using early continue pattern to reduce nesting.
49:25: Consider using early continue pattern to reduce nesting.
51:22: Consider using early continue pattern to reduce nesting.
52:12: Consider using early continue pattern to reduce nesting.
53:11: Consider using early continue pattern to reduce nesting.
54:13: Consider using early continue pattern to reduce nesting.
55:12: Consider using early continue pattern to reduce nesting.
56:6: Consider using early continue pattern to reduce nesting.
57:4: Consider using early continue pattern to reduce nesting.
