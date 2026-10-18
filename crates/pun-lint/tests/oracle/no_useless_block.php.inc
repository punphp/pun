<?php
function a() {
    foo();
    {
        // lead
        bar();
        if ($x) { { baz(); } }
    }
    switch ($x) {
        case 1: { qux(); }
    }
    {}
}
switch ($x) {
    default: { quux(); }
}
if ($x):
    { a1(); }
elseif ($y):
    { a2(); }
else:
    { a3(); }
endif;
if ($x):
    { a4(); }
else:
    a5();
endif;
if ($x):
    { a6(); }
endif;
while ($x):
    { b(); }
endwhile;
for (;;):
    { c(); }
endfor;
foreach ($a as $b):
    { d(); }
endforeach;
declare(ticks=1):
    { e(); }
enddeclare;
function z() {
    {
        foo();
        // c
    }
    bar();
}
-----
<?php
function a() {
    foo();
    // lead
    bar();
    if ($x) { baz(); }
    switch ($x) {
        case 1: qux();
    }
}
switch ($x) {
    default: quux();
}
if ($x):
    a1();
elseif ($y):
    a2();
else:
    a3();
endif;
if ($x):
    a4();
else:
    a5();
endif;
if ($x):
    a6();
endif;
while ($x):
    b();
endwhile;
for (;;):
    c();
endfor;
foreach ($a as $b):
    d();
endforeach;
declare(ticks=1):
    e();
enddeclare;
function z() {
    foo();
    // c

    bar();
}
