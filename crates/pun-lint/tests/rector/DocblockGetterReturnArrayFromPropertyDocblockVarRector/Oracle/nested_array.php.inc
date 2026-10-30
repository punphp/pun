<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var int[][]
     */
    private array $items = [];

    public function getItems(): array
    {
        return $this->items;
    }
}
-----
<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var int[][]
     */
    private array $items = [];

    /**
     * @return int[][]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
