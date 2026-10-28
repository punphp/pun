<?php

final class SomeClass
{
    /**
     * @var array<int, $this>
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
     * @var array<int, $this>
     */
    private array $items = [];

    /**
     * @return array<int, $this>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
