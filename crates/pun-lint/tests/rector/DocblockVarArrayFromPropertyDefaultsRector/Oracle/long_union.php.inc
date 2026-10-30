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
     * @var int[]|string[]|float[]|bool[]|array<string|int, string|int[]|int>[]|null[]
     */
    private array $items = [1, 'a', 1.5, true, null, ['x'], ['y' => 2], [[1]]];
}
