<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var array<int, true>
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
     * @var array<int, true>
     */
    private array $items = [];

    /**
     * @return array<int, true>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
