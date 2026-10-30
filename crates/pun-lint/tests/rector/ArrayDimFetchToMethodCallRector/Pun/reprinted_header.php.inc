<?php

// When an `unset()` that is the only statement of its list becomes several statements, php-parser's printer reprints
// the node that holds the list with its pretty printer: `while ($x) {`, `function (SomeClass $o) use ($x) {`, a
// function's `{` on a line of its own. Pun keeps that header as written and writes the list as the printer does, a
// line per statement; the laravel preset formats both alike.

class SomeClass {}

class K {
    public function m( SomeClass $o ,$x ) : void {
        while( $x ){ unset($o['a'], $o['b']); }
        if( $x ){
            unset($o['a'], $o['b']);
        }elseif($x){ $x = 1; }
    }
}

function g(SomeClass $o) {
    /** doc */
    unset($o['a'], $o['b']);
}

$f = function( SomeClass $o ) use( $x ){
    unset($o['a'], $o['b']);
};
-----
<?php

// When an `unset()` that is the only statement of its list becomes several statements, php-parser's printer reprints
// the node that holds the list with its pretty printer: `while ($x) {`, `function (SomeClass $o) use ($x) {`, a
// function's `{` on a line of its own. Pun keeps that header as written and writes the list as the printer does, a
// line per statement; the laravel preset formats both alike.

class SomeClass {}

class K {
    public function m( SomeClass $o ,$x ) : void {
        while( $x ){
            $o->unset('a');
            $o->unset('b');
        }
        if( $x ){
            $o->unset('a');
            $o->unset('b');
        }elseif($x){ $x = 1; }
    }
}

function g(SomeClass $o) {
    $o->unset('a');
    $o->unset('b');
}

$f = function( SomeClass $o ) use( $x ){
    $o->unset('a');
    $o->unset('b');
};
