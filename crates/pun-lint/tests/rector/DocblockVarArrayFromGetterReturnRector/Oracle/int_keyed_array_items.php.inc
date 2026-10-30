<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, array<int, string>>
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
     * @var array<int, string[]>
     */
    private array $items = [];

    /**
     * @return array<int, array<int, string>>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
