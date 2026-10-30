<?php

namespace App;

enum Suit
{
    case Hearts;

    public function oldCase() {}
}

class B
{
}

class T extends B
{
    private function old() {}

    public function new() {}

    public function f(string $c)
    {
        $this->old();
        $c::Hearts->oldCase();
        Suit::Hearts->oldCase();
    }
}

interface I
{
    public function new();
}

class Q
{
}

abstract class R extends Q implements I
{
    public function old() {}
}

function g(R $r)
{
    $r->old();
}
-----
<?php

namespace App;

enum Suit
{
    case Hearts;

    public function oldCase() {}
}

class B
{
}

class T extends B
{
    private function old() {}

    public function new() {}

    public function f(string $c)
    {
        $this->old();
        $c::Hearts->oldCase();
        Suit::Hearts->newCase();
    }
}

interface I
{
    public function new();
}

class Q
{
}

abstract class R extends Q implements I
{
    public function new() {}
}

function g(R $r)
{
    $r->old();
}
