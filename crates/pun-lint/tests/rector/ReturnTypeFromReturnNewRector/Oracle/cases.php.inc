<?php

namespace App;

use App\Models\Thing as Alias;

class Thing
{
}

class Other extends Thing
{
}

function make()
{
    return new Thing();
}

function makeAlias()
{
    return new Alias();
}

function makeUnknown()
{
    return new \Vendor\Missing();
}

function makeEither($a)
{
    if ($a) {
        return new Thing();
    }

    return new Other();
}

function makeSameTwice($a)
{
    if ($a) {
        return new Thing();
    }

    return new Thing;
}

function makeVariable()
{
    $thing = new Thing();
    $thing->name = 'x';
    $thing['offset'] = 1;

    return $thing;
}

function makeReassigned($a)
{
    $thing = new Thing();
    [$thing] = [$a];

    return $thing;
}

function makeForeach($items)
{
    $thing = new Thing();
    foreach ($items as $thing) {
    }

    return $thing;
}

function makeReference()
{
    $thing = new Thing();
    $ref = &$thing;

    return $thing;
}

function makeAnonymous()
{
    return new class extends Thing {
    };
}

function makeAnonymousTwo()
{
    return new class extends Thing implements \Countable {
        public function count(): int
        {
            return 0;
        }
    };
}

function makeAnonymousVariable()
{
    $x = new class implements \Countable {
        public function count(): int
        {
            return 0;
        }
    };

    return $x;
}

function makeDynamic($class)
{
    return new $class();
}

function makeInternal()
{
    return new \ArrayObject();
}

final class Factory
{
    public function make()
    {
        return new static();
    }

    public function makeSelf()
    {
        return new self();
    }

    public function makeMixed($a)
    {
        if ($a) {
            return new self();
        }

        return new Thing();
    }

    public function makeParent()
    {
        return new parent();
    }

    public function viaVariableStatic()
    {
        $x = new static();

        return $x;
    }
}
-----
<?php

namespace App;

use App\Models\Thing as Alias;

class Thing
{
}

class Other extends Thing
{
}

function make(): \App\Thing
{
    return new Thing();
}

function makeAlias()
{
    return new Alias();
}

function makeUnknown()
{
    return new \Vendor\Missing();
}

function makeEither($a): \App\Thing|\App\Other
{
    if ($a) {
        return new Thing();
    }

    return new Other();
}

function makeSameTwice($a): \App\Thing
{
    if ($a) {
        return new Thing();
    }

    return new Thing;
}

function makeVariable()
{
    $thing = new Thing();
    $thing->name = 'x';
    $thing['offset'] = 1;

    return $thing;
}

function makeReassigned($a)
{
    $thing = new Thing();
    [$thing] = [$a];

    return $thing;
}

function makeForeach($items)
{
    $thing = new Thing();
    foreach ($items as $thing) {
    }

    return $thing;
}

function makeReference(): \App\Thing
{
    $thing = new Thing();
    $ref = &$thing;

    return $thing;
}

function makeAnonymous(): \App\Thing
{
    return new class extends Thing {
    };
}

function makeAnonymousTwo(): object
{
    return new class extends Thing implements \Countable {
        public function count(): int
        {
            return 0;
        }
    };
}

function makeAnonymousVariable(): object
{
    $x = new class implements \Countable {
        public function count(): int
        {
            return 0;
        }
    };

    return $x;
}

function makeDynamic($class)
{
    return new $class();
}

function makeInternal(): \ArrayObject
{
    return new \ArrayObject();
}

final class Factory
{
    public function make(): static
    {
        return new static();
    }

    public function makeSelf(): self
    {
        return new self();
    }

    public function makeMixed($a): self|\App\Thing
    {
        if ($a) {
            return new self();
        }

        return new Thing();
    }

    public function makeParent()
    {
        return new parent();
    }

    public function viaVariableStatic()
    {
        $x = new static();

        return $x;
    }
}
