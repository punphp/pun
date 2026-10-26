<?php

class StaticConstants
{
    public const A = 0;

    final public const B = 0;

    public const int C = 0;

    public function lateStatic()
    {
        return static::A;
    }

    public function finalLateStatic()
    {
        return static::B;
    }

    public function typedLateStatic()
    {
        return static::C;
    }

    public function own()
    {
        return self::A;
    }

    /**
     * @var string
     */
    public const D = 'd';

    /**
     * The version.
     */
    public const E = 'e';

    public function documented()
    {
        return static::D;
    }

    public function described()
    {
        return static::E;
    }
}

class ObjectConstants
{
    public const A = 0;

    public function fromThis()
    {
        return $this::A;
    }

    public function anonymous()
    {
        return new class {
            public const B = 'b';

            public function late()
            {
                return static::B;
            }
        };
    }
}

final class FinalStatic
{
    public const A = 0;

    public function late()
    {
        return static::A;
    }
}

enum Suit
{
    const A = 1;

    public function late()
    {
        return static::A;
    }
}
-----
<?php

class StaticConstants
{
    public const A = 0;

    final public const B = 0;

    public const int C = 0;

    public function lateStatic()
    {
        return static::A;
    }

    public function finalLateStatic(): int
    {
        return static::B;
    }

    public function typedLateStatic(): int
    {
        return static::C;
    }

    public function own(): int
    {
        return self::A;
    }

    /**
     * @var string
     */
    public const D = 'd';

    /**
     * The version.
     */
    public const E = 'e';

    public function documented(): string
    {
        return static::D;
    }

    public function described()
    {
        return static::E;
    }
}

class ObjectConstants
{
    public const A = 0;

    public function fromThis()
    {
        return $this::A;
    }

    public function anonymous()
    {
        return new class {
            public const B = 'b';

            public function late(): string
            {
                return static::B;
            }
        };
    }
}

final class FinalStatic
{
    public const A = 0;

    public function late(): int
    {
        return static::A;
    }
}

enum Suit
{
    const A = 1;

    public function late(): int
    {
        return static::A;
    }
}
