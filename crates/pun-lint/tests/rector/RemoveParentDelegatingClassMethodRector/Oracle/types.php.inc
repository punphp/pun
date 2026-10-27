<?php

namespace Pun\Oracle\Override2;

enum Suit
{
    case Hearts;
}

interface A
{
}

interface B
{
}

abstract class Base
{
    public function iter(iterable $a): iterable
    {
        return $a;
    }

    public function call(callable $a): callable
    {
        return $a;
    }

    public function mix(mixed $a): mixed
    {
        return $a;
    }

    public function obj(object $a): object
    {
        return $a;
    }

    public function suit(Suit $a): Suit
    {
        return $a;
    }

    public function nothing(): null
    {
        return null;
    }

    public function fails(): never
    {
        throw new \RuntimeException();
    }

    public function union(int|string $a)
    {
    }

    public function both(A&B $a)
    {
    }

    public function one($a)
    {
    }

    public function comment()
    {
    }

    public function says($a)
    {
    }

    public function dynamic($a)
    {
    }
}

class Child extends Base
{
    public function iter(iterable $a): iterable
    {
        return parent::iter($a);
    }

    public function call(callable $a): callable
    {
        return parent::call($a);
    }

    public function mix(mixed $a): mixed
    {
        return parent::mix($a);
    }

    public function obj(object $a): object
    {
        return parent::obj($a);
    }

    public function suit(Suit $a): Suit
    {
        return parent::suit($a);
    }

    public function nothing(): null
    {
        return parent::nothing();
    }

    public function fails(): never
    {
        parent::fails();
    }

    public function union(string|int $a)
    {
        parent::union($a);
    }

    public function both(A&B $a)
    {
        parent::both($a);
    }

    public function one($a, $b)
    {
        parent::one($a, $b);
    }

    public function comment()
    {
        // only a comment
    }

    public function says($a)
    {
        echo $a;
    }

    public function dynamic($a)
    {
        $a::dynamic($a);
    }
}

class UnknownParent extends \Pun\Oracle\Missing\Nowhere
{
    public function run()
    {
        parent::run();
    }
}

class MissingMethod extends Base
{
    const X = 1;

    public function missing()
    {
        parent::missing();
    }
}
-----
<?php

namespace Pun\Oracle\Override2;

enum Suit
{
    case Hearts;
}

interface A
{
}

interface B
{
}

abstract class Base
{
    public function iter(iterable $a): iterable
    {
        return $a;
    }

    public function call(callable $a): callable
    {
        return $a;
    }

    public function mix(mixed $a): mixed
    {
        return $a;
    }

    public function obj(object $a): object
    {
        return $a;
    }

    public function suit(Suit $a): Suit
    {
        return $a;
    }

    public function nothing(): null
    {
        return null;
    }

    public function fails(): never
    {
        throw new \RuntimeException();
    }

    public function union(int|string $a)
    {
    }

    public function both(A&B $a)
    {
    }

    public function one($a)
    {
    }

    public function comment()
    {
    }

    public function says($a)
    {
    }

    public function dynamic($a)
    {
    }
}

class Child extends Base
{
    public function fails(): never
    {
        parent::fails();
    }

    public function union(string|int $a)
    {
        parent::union($a);
    }

    public function one($a, $b)
    {
        parent::one($a, $b);
    }

    public function comment()
    {
        // only a comment
    }

    public function says($a)
    {
        echo $a;
    }

    public function dynamic($a)
    {
        $a::dynamic($a);
    }
}

class UnknownParent extends \Pun\Oracle\Missing\Nowhere
{
    public function run()
    {
        parent::run();
    }
}

class MissingMethod extends Base
{
    const X = 1;

    public function missing()
    {
        parent::missing();
    }
}
