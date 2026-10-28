<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return int[]|true|false
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
     * @var int[]|bool
     */
    private array $items = [];

    /**
     * @return int[]|true|false
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
