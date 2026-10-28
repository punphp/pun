<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, void>
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
     * @var void[]
     */
    private array $items = [];

    /**
     * @return array<int, void>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
