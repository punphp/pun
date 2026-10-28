<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var list<array{int, string}>
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
     * @var list<array{int, string}>
     */
    private array $items = [];

    /**
     * @return list<array{int, string}>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
