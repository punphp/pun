<?php

final class SomeClass
{
    /**
     * @var array<int, int|string>[]
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
    /**
     * @var array<int, int|string>[]
     */
    private array $items = [];

    /**
     * @return array<int, int|string>[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
