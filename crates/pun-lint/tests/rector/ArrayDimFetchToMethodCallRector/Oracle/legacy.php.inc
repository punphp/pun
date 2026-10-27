<?php

class SomeClass {}

function f(SomeClass $o) {
    $a = array($o['x']);
    $b = array('k' => $o['y']);
}
-----
<?php

class SomeClass {}

function f(SomeClass $o) {
    $a = array($o->get('x'));
    $b = array('k' => $o->get('y'));
}
