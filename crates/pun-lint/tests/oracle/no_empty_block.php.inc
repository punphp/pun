<?php
if ($a) { ; }
if ($b) {
    // todo
}
if ($c) {
} // after
if ($d) /* x */ {}
foreach ($a as $k => $v) ;
if ($e):
endif;
if ($f) {} else foo();
if ($g) {} else { /* only */ }
if ($h): else: bar(); endif;
if ($i):
    // c
else:
    baz();
endif;
foo();
if ($j) {
}
function f($a, $b, $x, $y) {
if ($a instanceof B) {} else { x(1); }
if ($a = foo()) {} else { x(2); }
if ($a ?? $b) {} else { x(3); }
if ($a || $b) {} else { x(4); }
if ($a && $b) {} else { x(5); }
if ($a===$b) {} else { x(6); }
if (($a) <> $b) {} else { x(7); }
if (!($a && $b)) {} else { x(8); }
if (foo()) {} else { x(9); }
if ($a + 1) {} else { x(10); }
if ($a or $b) {} else { x(11); }
if (-$a) {} else { x(12); }
if (($a || $b) === $c) {} else { x(13); }
if ($a > 1 && $b) {} else { x(14); }
if ((bool) $a) {} else { x(15); }
if (@foo()) {} else { x(17); }
if ($a) {} else if ($b) { x(18); }
if ($a) {} elseif ($b) { x(19); }
if ($a);
if ($a); else x(20);
    for ($i = 0; $i < 3; $i++) {}
    echo $i;
    for ($j = 0; $j < 3; $j++) {}
    foreach ($items as $item) {}
    echo $item;
    foreach (gen() as $item) {}
    foreach ($this->items as $k => $v) {}
    echo $k;
    try {
        foreach ($list as $one) {}
    } catch (E $e) {
        echo $one;
    }
    if ($x = 5) {}
    if ($x === 5) {}
    switch ($x) {
        case 1:
            if ($y) {}
            break;
    }
}
if (foo()) {} else {}
if ($a !== $b) {} else { x(21); }
if ($a == $b) {} else { x(22); }
if ($a > $b) {} else { x(23); }
if ($a < $b) {} else { x(24); }
if ($a >= $b) {} else { x(25); }
if ($a <= $b) {} else { x(26); }
if ($a + 1 === $b . 'x') {} else { x(27); }
if (($a & 1) == 2) {} else { x(29); }
if ($a = &$b) {}
for (;;): endfor;
for ($$n = 0; ;) {}
echo $$m;
if (foo()) {
} else {
}
if (foo()):
else:
endif;
if (foo()) {} else { }
if (($a ** $b) === $c) {} else { x(30); }
if (($a instanceof $b) === $c) {} else { x(31); }
if (($a * $b) === $c) {} else { x(32); }
if (($a / $b) === $c) {} else { x(33); }
if (($a % $b) === $c) {} else { x(34); }
if (($a + $b) === $c) {} else { x(35); }
if (($a - $b) === $c) {} else { x(36); }
if (($a << $b) === $c) {} else { x(37); }
if (($a >> $b) === $c) {} else { x(38); }
if (($a . $b) === $c) {} else { x(39); }
if (($a < $b) === $c) {} else { x(40); }
if (($a <= $b) === $c) {} else { x(41); }
if (($a > $b) === $c) {} else { x(42); }
if (($a >= $b) === $c) {} else { x(43); }
if (($a & $b) === $c) {} else { x(44); }
if (($a ^ $b) === $c) {} else { x(45); }
if (($a | $b) === $c) {} else { x(46); }
if (($a && $b) === $c) {} else { x(47); }
if (($a ?? $b) === $c) {} else { x(48); }
if (($a and $b) === $c) {} else { x(49); }
if (($a xor $b) === $c) {} else { x(50); }
switch ($x) {
    case 1:
        if ($y) {}
    case 2:
        foo();
}
foreach ($a as $b): endforeach;
-----
<?php
if ($b) {
    // todo
}
foreach ($a as $k => $v) ;
if (!$f) {
    foo();
}
if (!$g) {
    /* only */
}
if (!$h) {
    bar();
}
if ($i):
    // c
else:
    baz();
endif;
foo();
function f($a, $b, $x, $y) {
if (!$a instanceof B) {
    x(1);
}
if (!$a = foo()) {
    x(2);
}
if (!($a ?? $b)) {
    x(3);
}
if (!($a || $b)) {
    x(4);
}
if (!($a && $b)) {
    x(5);
}
if ($a !== $b) {
    x(6);
}
if ($a == $b) {
    x(7);
}
if ($a && $b) {
    x(8);
}
if (!foo()) {
    x(9);
}
if (!($a + 1)) {
    x(10);
}
if (!($a or $b)) {
    x(11);
}
if (!-$a) {
    x(12);
}
if (!(($a || $b) === $c)) {
    x(13);
}
if (!($a > 1 && $b)) {
    x(14);
}
if (!(bool) $a) {
    x(15);
}
if (!@foo()) {
    x(17);
}
if (!$a) {
    if ($b) { x(18); }
}
if ($a) {} elseif ($b) { x(19); }
if (!$a) {
    x(20);
}
    for ($i = 0; $i < 3; $i++) {}
    echo $i;
    foreach ($items as $item) {}
    echo $item;
    foreach (gen() as $item) {}
    echo $k;
    try {
        foreach ($list as $one) {}
    } catch (E $e) {
        echo $one;
    }
    if ($x = 5) {}
    switch ($x) {
        case 1:
            break;
    }
}
if (!foo()) {}
if ($a === $b) {
    x(21);
}
if ($a != $b) {
    x(22);
}
if ($a <= $b) {
    x(23);
}
if ($a >= $b) {
    x(24);
}
if ($a < $b) {
    x(25);
}
if ($a > $b) {
    x(26);
}
if ($a + 1 !== $b . 'x') {
    x(27);
}
if (($a & 1) != 2) {
    x(29);
}
for ($$n = 0; ;) {}
echo $$m;
if (!foo()) {
}
if (!foo()):
endif;
if (!foo()) {}
if ($a ** $b !== $c) {
    x(30);
}
if ($a instanceof $b !== $c) {
    x(31);
}
if ($a * $b !== $c) {
    x(32);
}
if ($a / $b !== $c) {
    x(33);
}
if ($a % $b !== $c) {
    x(34);
}
if ($a + $b !== $c) {
    x(35);
}
if ($a - $b !== $c) {
    x(36);
}
if ($a << $b !== $c) {
    x(37);
}
if ($a >> $b !== $c) {
    x(38);
}
if ($a . $b !== $c) {
    x(39);
}
if ($a < $b !== $c) {
    x(40);
}
if ($a <= $b !== $c) {
    x(41);
}
if ($a > $b !== $c) {
    x(42);
}
if ($a >= $b !== $c) {
    x(43);
}
if (($a & $b) !== $c) {
    x(44);
}
if (($a ^ $b) !== $c) {
    x(45);
}
if (($a | $b) !== $c) {
    x(46);
}
if (($a && $b) !== $c) {
    x(47);
}
if (($a ?? $b) !== $c) {
    x(48);
}
if (($a and $b) !== $c) {
    x(49);
}
if (($a xor $b) !== $c) {
    x(50);
}
switch ($x) {
    case 1:
    case 2:
        foo();
}
