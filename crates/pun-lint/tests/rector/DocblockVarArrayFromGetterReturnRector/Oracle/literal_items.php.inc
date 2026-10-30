<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, 'a'|'b'>
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
     * @var string[]
     */
    private array $items = [];

    /**
     * @return array<int, 'a'|'b'>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
