<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, \UnitEnum>
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
     * @var \UnitEnum[]
     */
    private array $items = [];

    /**
     * @return array<int, \UnitEnum>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
