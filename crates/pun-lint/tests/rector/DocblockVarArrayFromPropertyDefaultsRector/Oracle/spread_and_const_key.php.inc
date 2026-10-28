<?php

class SomeClass
{
    const OTHER = [1];

    const KEY = 'k';

    private array $items = [...self::OTHER];

    private array $keyed = [self::KEY => 1];
}
-----
<?php

class SomeClass
{
    const OTHER = [1];

    const KEY = 'k';

    /**
     * @var int[]
     */
    private array $items = [...self::OTHER];

    /**
     * @var array<string, int>
     */
    private array $keyed = [self::KEY => 1];
}
