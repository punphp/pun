<?php

namespace App;

interface Named
{
    public function name(): string;
}

interface Labelled extends Named
{
    public function label();
}

class Base
{
    public function size(): int
    {
        return 1;
    }

    private function secret(): int
    {
        return 1;
    }
}

class Middle extends Base
{
    public function make(): parent
    {
        return new Base();
    }

    public function size(): int
    {
        return 2;
    }
}

final class Leaf extends Middle
{
    public function make()
    {
        return new Base();
    }

    private function secret()
    {
        return 3;
    }
}

trait Sized
{
    abstract public function length(): int;

    public function width(): int
    {
        return 1;
    }
}

final class Box
{
    use Sized;

    public function length()
    {
        return 1;
    }

    public function width()
    {
        return 2;
    }

    public function depth()
    {
        return 3;
    }
}
-----
<?php

namespace App;

interface Named
{
    public function name(): string;
}

interface Labelled extends Named
{
    public function label();
}

class Base
{
    public function size(): int
    {
        return 1;
    }

    private function secret(): int
    {
        return 1;
    }
}

class Middle extends Base
{
    public function make(): parent
    {
        return new Base();
    }

    public function size(): int
    {
        return 2;
    }
}

final class Leaf extends Middle
{
    public function make(): \App\Base
    {
        return new Base();
    }

    private function secret()
    {
        return 3;
    }
}

trait Sized
{
    abstract public function length(): int;

    public function width(): int
    {
        return 1;
    }
}

final class Box
{
    use Sized;

    public function length(): int
    {
        return 1;
    }

    public function width()
    {
        return 2;
    }

    public function depth()
    {
        return 3;
    }
}
