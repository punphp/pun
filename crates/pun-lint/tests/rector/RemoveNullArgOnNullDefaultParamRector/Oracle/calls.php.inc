<?php

namespace App;

function withNulls(?int $a = null, ?int $b = null, int $c = 1, ?string $d = null)
{
}

function readsArgs($a = null)
{
    return func_get_args();
}

function variadic($a = null, ...$rest)
{
}

final class Box
{
    public function __construct(?int $id = null)
    {
    }

    public function put(?int $a = null, ?int $b = null)
    {
    }

    public static function make(?int $a = null)
    {
    }

    public function run(array $args)
    {
        $this->put(null, null);
        $this->put(1, null);
        $this->put(null, 1);
        self::make(null);
        static::make(NULL);
        $this->put(...$args);
        $this->put(null, b: null);
        new Box(null);
        new Box;
        new Box();
        $this->unknown(null);
        $other->put(null);
    }
}

withNulls(null, null, 1, null);
withNulls(null, null, null);
withNulls(1, 2, 3, \null);
withNulls(
    1,
    null,
);
readsArgs(null);
variadic(null);
\App\withNulls(null);
strlen(null);
-----
<?php

namespace App;

function withNulls(?int $a = null, ?int $b = null, int $c = 1, ?string $d = null)
{
}

function readsArgs($a = null)
{
    return func_get_args();
}

function variadic($a = null, ...$rest)
{
}

final class Box
{
    public function __construct(?int $id = null)
    {
    }

    public function put(?int $a = null, ?int $b = null)
    {
    }

    public static function make(?int $a = null)
    {
    }

    public function run(array $args)
    {
        $this->put();
        $this->put(1);
        $this->put(null, 1);
        self::make();
        static::make();
        $this->put(...$args);
        $this->put(null, b: null);
        new Box();
        new Box;
        new Box();
        $this->unknown(null);
        $other->put(null);
    }
}

withNulls(null, null, 1);
withNulls(null, null, null);
withNulls(1, 2, 3);
withNulls(
    1,
);
readsArgs(null);
variadic(null);
\App\withNulls();
strlen(null);
