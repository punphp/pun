<?php
while (1) { foo(); continue; // trailing
}
while (2) { foo(); continue; /* c */ }
foreach ($a as $b):
    foo();
    continue;
endforeach;
for (;;) { if ($x) { continue; } continue 1; }
while (3) { continue 0; }
while (4) { continue (1); }
while (5) { continue; ; }
foreach ($a as $b) continue;
while (6): foo(); continue; endwhile;
for (;;): continue; endfor;
do { foo(); continue; } while (7);
do continue; while (9);
while (8) foo();
for (;;): foo(); endfor;
while (10): foo(); endwhile;
for ($i=0;$i<3;$i++): continue; endfor;
foreach ($a as $k => $v): continue; endforeach;
while(1):
    continue;
endwhile;
function f() {
    foreach ($a as $b):
        continue;
    endforeach;
}
-----
<?php
while (1) { foo(); continue; // trailing
}
while (2) { foo(); continue; /* c */ }
foreach ($a as $b):
    foo();
endforeach;
for (;;) { if ($x) { continue; } }
while (3) {
}
while (4) {
}
while (5) {
}
foreach ($a as $b) {
}
while (6): foo(); endwhile;
for (;;) {
}
do { foo(); } while (7);
do {
} while (9);
while (8) foo();
for (;;): foo(); endfor;
while (10): foo(); endwhile;
for ($i=0; $i<3; $i++) {
}
foreach ($a as $k => $v) {
}
while (1) {
}
function f() {
    foreach ($a as $b) {
    }
}
