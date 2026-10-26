<?php
namespace App {
    function f() {}
    /** doc */
    // between
    # hash
    /* multi */
    function g() {}
    /** doc */ $x = 1; function h() {}
    class C {
        const A = 1, B = 2;
        /** d */ public $p, $q;
        public $r { get => 1; }
        public static function m() {}
        abstract function n();
    }
    interface I { const X = 1; function m(); }
    trait T { public $t; function m() {} }
    enum E { case A; /** d */ case B; function m() {} }
    const K = 1;
    static $s = 1, $t;
    $c = new class { function m() {} };
    function outer() { function inner() {} }
}
