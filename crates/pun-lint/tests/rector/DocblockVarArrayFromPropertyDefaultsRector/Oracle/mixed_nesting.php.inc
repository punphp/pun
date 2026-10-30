<?php

class SomeClass
{
    private array $items = ['a' => [1], 'b' => ['x']];
}
-----
<?php

class SomeClass
{
    /**
     * @var array<string, array<string|int>>
     */
    private array $items = ['a' => [1], 'b' => ['x']];
}
