<?php

final class SomeClass
{
    /**
     * @var array<int, "a\\b"|array{"c d": int}>
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
     * @var array<int, "a\\b"|array{"c d": int}>
     */
    private array $items = [];

    /**
     * @return array<int, "a\\b"|array{"c d": int}>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
