<?php

namespace App;

interface Bag {}

class SomeClass implements \ArrayAccess
{
    public function offsetExists($k): bool { return true; }
    public function offsetGet($k): mixed { return null; }
    public function offsetSet($k, $v): void {}
    public function offsetUnset($k): void {}
}

class Child extends \SomeClass {}

class Container {}

function f(\SomeClass $a, ?\SomeClass $b, \SomeClass|false $c, \SomeClass|int $d, SomeClass $e, Child $f, \Container $g, array $h, $i)
{
    $a['k'];
    $b['k'];
    $c['k'];
    $d['k'];
    $e['k'];
    $f['k'];
    $g['k'];
    $g['k'] = 1;
    isset($g['k']);
    unset($g['k']);
    $h['k'];
    $i['k'];
}
-----
<?php

namespace App;

interface Bag {}

class SomeClass implements \ArrayAccess
{
    public function offsetExists($k): bool { return true; }
    public function offsetGet($k): mixed { return null; }
    public function offsetSet($k, $v): void {}
    public function offsetUnset($k): void {}
}

class Child extends \SomeClass {}

class Container {}

function f(\SomeClass $a, ?\SomeClass $b, \SomeClass|false $c, \SomeClass|int $d, SomeClass $e, Child $f, \Container $g, array $h, $i)
{
    $a->get('k');
    $b->get('k');
    $c->get('k');
    $d['k'];
    $e['k'];
    $f['k'];
    $g->get('k');
    $g['k'] = 1;
    isset($g['k']);
    unset($g['k']);
    $h['k'];
    $i['k'];
}
