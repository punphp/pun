<?php

final class SomeClass
{
    /**
     * @var array<int, array{'a b': int}>
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
     * @var array<int, array{'a b': int}>
     */
    private array $items = [];

    /**
     * @return array<int, array{'a b': int}>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
