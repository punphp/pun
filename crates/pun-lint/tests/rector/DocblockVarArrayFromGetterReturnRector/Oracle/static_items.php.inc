<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return static[]
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
     * @var static[]
     */
    private array $items = [];

    /**
     * @return static[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
