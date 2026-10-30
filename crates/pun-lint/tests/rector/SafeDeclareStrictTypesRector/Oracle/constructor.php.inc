<?php

namespace Pun\Oracle\SafeStrict;

class A
{
    public function __construct(int $a)
    {
    }

    public static function make(): static
    {
        return new static(1);
    }
}

new A(1);
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

class A
{
    public function __construct(int $a)
    {
    }

    public static function make(): static
    {
        return new static(1);
    }
}

new A(1);
