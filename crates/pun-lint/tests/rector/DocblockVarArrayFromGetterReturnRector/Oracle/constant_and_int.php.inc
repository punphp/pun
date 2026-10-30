<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return int[]|1|int
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
     * @var int[]|int
     */
    private array $items = [];

    /**
     * @return int[]|1|int
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
