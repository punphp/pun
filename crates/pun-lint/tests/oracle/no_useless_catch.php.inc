<?php
function a() {
    try {
        foo();
        // after
    } catch (E $e) {
        throw $e;
    }
    try {
        $x = <<<TXT
          body
        TXT;
    } catch (E $e) {
        throw ($e);
        bar();
    }
    try {
    } finally {
        // c
    }
    try { foo(); } catch (E) { throw $e; }
    try { foo(); } catch (A | B $e) { throw $e; }
    if ($x) try { foo(); } catch (E $e) { throw $e; }
}
function a() {
    try {
        foo();
    } catch (E $e) {
        throw $e;
    }
    try {
        bar();
        baz();
    } catch (E $e) {
        throw $e;
    }
    qux();
}
function b() {
    try {
        foo();
        // after
    } catch (E $e) {
        throw $e;
    }
    qux();
}
function c() {
    try {
        // before
        foo();
    } catch (E $e) {
        throw $e;
    }
}
function d() {
    try { foo(); } catch (E $e) { if ($x) {} throw $e; }
    if ($x) try { foo(); bar(); } catch (E $e) { throw $e; }
}
function e() {
    if ($x) try { foo(); bar(); // c
    } catch (E $e) { throw $e; }
    if ($x) try {
            foo();
            bar();
        } catch (E $e) { throw $e; }
}
-----
<?php
function a() {
    foo();
    // after

    $x = <<<TXT
          body
        TXT;
    try { foo(); } catch (E) { throw $e; }
    foo();
    if ($x) foo();
}
function a() {
    foo();
    bar();
    baz();
    qux();
}
function b() {
    foo();
    // after

    qux();
}
function c() {
    // before
    foo();
}
function d() {
    try { foo(); } catch (E $e) { if ($x) {} throw $e; }
    if ($x) {
        foo();
        bar();
    }
}
function e() {
    if ($x) {
        foo();
        bar();
        // c
    }
    if ($x) {
        foo();
        bar();
    }
}
