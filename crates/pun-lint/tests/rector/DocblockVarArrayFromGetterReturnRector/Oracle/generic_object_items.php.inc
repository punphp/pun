<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, \ArrayObject<int, \SomeClass>>
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
     * @var \ArrayObject<int, \SomeClass>[]
     */
    private array $items = [];

    /**
     * @return array<int, \ArrayObject<int, \SomeClass>>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
