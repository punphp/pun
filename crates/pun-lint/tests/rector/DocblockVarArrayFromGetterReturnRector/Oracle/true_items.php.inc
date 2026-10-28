<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, true>
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
     * @var bool[]
     */
    private array $items = [];

    /**
     * @return array<int, true>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
