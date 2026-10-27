<?php

// In a string, `$o[key]` reads the key 'key' and `$o[012]` the key '012'. Rector writes `{$o->get(key)}`, the
// undefined constant `key` (an Error in PHP 8), and `{$o->get(012)}`, the integer 10. Pun quotes such a key.
// `${o['x']}` is the variable named by `o['x']`, no access on `$o`; Rector writes `{o->get('x')}` for it, Pun
// leaves it alone.

class SomeClass {}

function f(SomeClass $o) {
    echo "a $o[key] b $o[012]";
    echo "${o['x']}";
}
-----
<?php

// In a string, `$o[key]` reads the key 'key' and `$o[012]` the key '012'. Rector writes `{$o->get(key)}`, the
// undefined constant `key` (an Error in PHP 8), and `{$o->get(012)}`, the integer 10. Pun quotes such a key.
// `${o['x']}` is the variable named by `o['x']`, no access on `$o`; Rector writes `{o->get('x')}` for it, Pun
// leaves it alone.

class SomeClass {}

function f(SomeClass $o) {
    echo "a {$o->get('key')} b {$o->get('012')}";
    echo "${o['x']}";
}
