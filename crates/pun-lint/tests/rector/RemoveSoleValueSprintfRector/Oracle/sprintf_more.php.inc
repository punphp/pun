<?php

$r = sprintf('%s', $top);

function a(string $s, $p, $items)
{
    $r = printf('%s', 'a');
    $r = sprintf('%s');
    $r = sprintf(...$p);
    $r = sprintf('%s', ...$p);
    $r = sprintf($p, 'a');
    $r = sprintf('%d', 'a');
    $r = sprintf('%s', A::class);
    $r = sprintf('%s', A::FOO);
    $r = sprintf('%s', "a $p");
    $r = sprintf('%s', `ls`);
    $r = sprintf('%s', 1);
    $r = sprintf('%s', $p);
    $r = sprintf('%s', $s);
    $l = 'a';
    $r = sprintf('%s', $l);
    $m = 'a';
    foreach ($items as $m) {
    }
    $r = sprintf('%s', $m);
    $n = 'a';
    $ref = &$other;
    $r = sprintf('%s', $n);
    $r = sprintf('%s', $undefined);
}

$closure = function () use ($top) {
    return sprintf('%s', $top);
};

class A
{
    public function m(string $s)
    {
        return sprintf('%s', $s);
    }
}
-----
<?php

$r = sprintf('%s', $top);

function a(string $s, $p, $items)
{
    $r = printf('%s', 'a');
    $r = sprintf('%s');
    $r = sprintf(...$p);
    $r = sprintf('%s', ...$p);
    $r = sprintf($p, 'a');
    $r = sprintf('%d', 'a');
    $r = A::class;
    $r = sprintf('%s', A::FOO);
    $r = "a $p";
    $r = sprintf('%s', `ls`);
    $r = sprintf('%s', 1);
    $r = sprintf('%s', $p);
    $r = $s;
    $l = 'a';
    $r = $l;
    $m = 'a';
    foreach ($items as $m) {
    }
    $r = sprintf('%s', $m);
    $n = 'a';
    $ref = &$other;
    $r = $n;
    $r = sprintf('%s', $undefined);
}

$closure = function () use ($top) {
    return sprintf('%s', $top);
};

class A
{
    public function m(string $s)
    {
        return $s;
    }
}
