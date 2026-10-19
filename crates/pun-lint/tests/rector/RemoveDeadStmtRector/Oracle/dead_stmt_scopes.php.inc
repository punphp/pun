<?php

$anon = new class {
    private $p;

    public function f()
    {
        $this->p;
    }
};

function members($a, $b)
{
    $a->$b;
    $a->{$b . 'c'};
    $a->b['c'];
}

$c = function (Foo $o, $s) {
    $s['a'];
};

$d = function () {
    $e = fn (Foo $o) => function () use ($o) {
        $o['a'];
    };
};
-----
<?php

$anon = new class {
    private $p;

    public function f()
    {
    }
};

function members($a, $b)
{
    $a->$b;
    $a->{$b . 'c'};
}

$c = function (Foo $o, $s) {
};

$d = function () {
    $e = fn (Foo $o) => function () use ($o) {
    };
};
