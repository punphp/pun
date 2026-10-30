<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return string[]|null
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
     * @var string[]|null
     */
    private array $items = [];

    /**
     * @return string[]|null
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
