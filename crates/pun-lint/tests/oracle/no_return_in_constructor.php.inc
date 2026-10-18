<?php
class A {
    public function __construct() {
        if ($a) { return "x$y"; }
        if ($c) {
            return static::X;
        }
        if ($d) {
            return $this->x = 1;
        }
        switch ($e) {
            case 1: return foo();
        }
    }
}
class B {
    public function __construct() {
        foo();
        return bar();
    }
}
class C {
    public function __construct() {
        return [1];
    }
    public function other() {
        return 1;
    }
}
interface I {
    public function __construct();
}
class Edge {
    public function __construct($x) {
        if ($x) return foo();
        if ($x) { bar(); return baz(); }
        if ($x) { return array(1, 2); }
        if ($x) { return <<<T
            plain
            T; }
        if ($x) { return [1, ...$x]; }
        if ($x) { return [$x => 1]; }
        if ($x) { return [1 => 1, 'a' => [2]]; }
        if ($x) { return -1; }
        if ($x) { return +1.5; }
        if ($x) { return baz(); qux(); }
        if ($x) { bar(); /* c */ return baz(); }
        if ($x) { bar();  return baz(); qux(); }
        switch ($x) { case 1: bar(); return baz(); }
        if ($x) {
            // lead
            return foo();
        }
        if ($x) {
            bar();
            // lead
            return foo();
        }
        if ($x) { /* a */ return baz(); }
        if ($x) { /* a */ return baz(); qux(); }
        if ($x) {
            bar(); return baz();
        }
        if ($x) { bar(); return baz();
        }
    }
}
-----
<?php
class A {
    public function __construct() {
        if ($a) {
            "x$y";
            return;
        }
        if ($c) {
            static::X;
            return;
        }
        if ($d) {
            $this->x = 1;
            return;
        }
        switch ($e) {
            case 1:
                foo();
                return;
        }
    }
}
class B {
    public function __construct() {
        foo();
        return bar();
    }
}
class C {
    public function __construct() {
        return;
    }
    public function other() {
        return 1;
    }
}
interface I {
    public function __construct();
}
class Edge {
    public function __construct($x) {
        if ($x) {
            foo();
            return;
        }
        if ($x) { bar();
        baz();
        return; }
        if ($x) { return; }
        if ($x) { return; }
        if ($x) {
            [1, ...$x];
            return;
        }
        if ($x) {
            [$x => 1];
            return;
        }
        if ($x) { return; }
        if ($x) { return; }
        if ($x) { return; }
        if ($x) { baz();
        return;
        qux(); }
        if ($x) { bar();
        baz();
        return; }
        if ($x) { bar();
        baz();
        return; qux(); }
        switch ($x) { case 1: bar();
        baz();
        return; }
        if ($x) {
            foo();
            return;
        }
        if ($x) {
            bar();
            foo();
            return;
        }
        if ($x) {
            baz();
            return;
        }
        if ($x) { baz();
        return;
        qux(); }
        if ($x) {
            bar();
            baz();
            return;
        }
        if ($x) { bar();
        baz();
        return;
        }
    }
}
