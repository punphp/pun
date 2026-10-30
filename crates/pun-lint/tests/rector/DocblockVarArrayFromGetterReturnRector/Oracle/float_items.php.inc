<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return float[]
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
     * @var float[]
     */
    private array $items = [];

    /**
     * @return float[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
