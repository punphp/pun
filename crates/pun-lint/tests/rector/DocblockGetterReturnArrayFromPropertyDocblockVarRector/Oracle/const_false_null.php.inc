<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var array<int, false|null>
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
     * @var array<int, false|null>
     */
    private array $items = [];

    /**
     * @return array<int, false|null>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
