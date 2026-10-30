<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, int<0, max>>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @var int[]
     */
    private array $items = [];

    /**
     * @return array<int, int<0, max>>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
