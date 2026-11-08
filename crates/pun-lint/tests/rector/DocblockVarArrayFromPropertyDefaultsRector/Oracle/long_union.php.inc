<?php

class SomeClass
{
    private array $items = [1, 'a', 1.5, true, null, ['x'], ['y' => 2], [[1]]];
}
-----
<?php

class SomeClass
{
    /**
     * @var array<array<int|string, int[]|int|string>|bool|float|int|string|null>
     */
    private array $items = [1, 'a', 1.5, true, null, ['x'], ['y' => 2], [[1]]];
}
