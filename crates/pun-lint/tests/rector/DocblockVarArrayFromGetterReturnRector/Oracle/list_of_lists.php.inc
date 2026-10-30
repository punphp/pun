<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return list<list<int>>
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
     * @var list<list<int>>
     */
    private array $items = [];

    /**
     * @return list<list<int>>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
