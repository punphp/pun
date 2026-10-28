<?php

namespace Oracle\AssertShapes;

enum Suit
{
    case Hearts;
    case Spades;
}

final class Child
{
}

final class Holder
{
    public function run(Child $c, int $n, string $class)
    {
        check($c instanceof Child);
        assert();
        assert($c);
        assert($c === $c);
        assert(make() instanceof Child);
        assert($c instanceof $class);
        assert($undefined instanceof Child);
        assert($$class instanceof Child);
        assert($n instanceof Child);
        $suit = Suit::Hearts;
        assert($suit instanceof Suit);
        echo 1;
    }
}

function check(bool $value)
{
}

function make(): Child
{
    return new Child();
}
-----
<?php

namespace Oracle\AssertShapes;

enum Suit
{
    case Hearts;
    case Spades;
}

final class Child
{
}

final class Holder
{
    public function run(Child $c, int $n, string $class)
    {
        check($c instanceof Child);
        assert();
        assert($c);
        assert($c === $c);
        assert(make() instanceof Child);
        assert($c instanceof $class);
        assert($undefined instanceof Child);
        assert($$class instanceof Child);
        assert($n instanceof Child);
        $suit = Suit::Hearts;
        echo 1;
    }
}

function check(bool $value)
{
}

function make(): Child
{
    return new Child();
}
