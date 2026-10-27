<?php

class SomeClass {}

function f(SomeClass $o, $k) {
    echo "a $o[0] b $o[-1] c $o[$k]";
    echo "{$o['x']}";
    echo <<<EOT
    v $o[1] {$o['y']}
    EOT;
}
-----
<?php

class SomeClass {}

function f(SomeClass $o, $k) {
    echo "a {$o->get(0)} b {$o->get(-1)} c {$o->get($k)}";
    echo "{$o->get('x')}";
    echo <<<EOT
    v {$o->get(1)} {$o->get('y')}
    EOT;
}
