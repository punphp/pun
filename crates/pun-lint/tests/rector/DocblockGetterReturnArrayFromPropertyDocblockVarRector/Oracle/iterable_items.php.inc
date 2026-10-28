<?php

final class SomeClass
{
    /**
     * @var iterable<int, string>[]
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
     * @var iterable<int, string>[]
     */
    private array $items = [];

    /**
     * @return iterable<int, string>[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
