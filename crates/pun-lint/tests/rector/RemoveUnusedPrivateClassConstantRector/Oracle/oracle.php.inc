<?php

namespace App;

trait LocalTrait
{
    public function t()
    {
        return self::FROM_TRAIT;
    }
}

class Base
{
    public function b()
    {
        return static::FROM_PARENT;
    }
}

final class Holder extends Base
{
    use LocalTrait;

    private const UNUSED = 1;
    private const FROM_TRAIT = 2;
    private const FROM_PARENT = 3;
    private const BY_NAME = 4;
    private const BY_THIS = 5;
    private const LOWER = 6;
    private const IN_OTHER = 7;
    public const PUBLIC_ONE = 9;
    protected const PROTECTED_ONE = 10;
    private const TWO = 1, THREE = 2;
    const IMPLICIT = 11;

    public function run($other)
    {
        return [Holder::BY_NAME, \App\Holder::BY_NAME, $this::BY_THIS, self::lower, $other::IN_OTHER];
    }
}

$anon = new class {
    private const ANON_USED = 1;
    private const ANON_UNUSED = 2;

    public function run()
    {
        return self::ANON_USED;
    }
};

enum Suit
{
    case Hearts;

    private const UNUSED_IN_ENUM = 1;
}

interface Contract
{
    const X = 1;
}

trait T2
{
    private const IN_TRAIT = 1;
}

class Dynamic
{
    private const D = 1;

    public function run($name)
    {
        return self::{$name};
    }
}
-----
<?php

namespace App;

trait LocalTrait
{
    public function t()
    {
        return self::FROM_TRAIT;
    }
}

class Base
{
    public function b()
    {
        return static::FROM_PARENT;
    }
}

final class Holder extends Base
{
    use LocalTrait;
    private const FROM_TRAIT = 2;
    private const BY_NAME = 4;
    private const BY_THIS = 5;
    private const LOWER = 6;
    public const PUBLIC_ONE = 9;
    protected const PROTECTED_ONE = 10;
    private const TWO = 1, THREE = 2;
    const IMPLICIT = 11;

    public function run($other)
    {
        return [Holder::BY_NAME, \App\Holder::BY_NAME, $this::BY_THIS, self::lower, $other::IN_OTHER];
    }
}

$anon = new class {
    private const ANON_USED = 1;

    public function run()
    {
        return self::ANON_USED;
    }
};

enum Suit
{
    case Hearts;
}

interface Contract
{
    const X = 1;
}

trait T2
{
    private const IN_TRAIT = 1;
}

class Dynamic
{
    private const D = 1;

    public function run($name)
    {
        return self::{$name};
    }
}
