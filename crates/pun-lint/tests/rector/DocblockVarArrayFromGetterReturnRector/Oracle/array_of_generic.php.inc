<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<string, int>[]
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
     * @var array<mixed, array<string, int>>
     */
    private array $items = [];

    /**
     * @return array<string, int>[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
