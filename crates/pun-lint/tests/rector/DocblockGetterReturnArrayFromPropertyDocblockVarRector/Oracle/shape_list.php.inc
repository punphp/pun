<?php

final class SomeClass
{
    /**
     * @var list<array{a: int, b?: string, ...}>
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
     * @var list<array{a: int, b?: string, ...}>
     */
    private array $items = [];

    /**
     * @return list<array{a: int, b?: string, ...}>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
