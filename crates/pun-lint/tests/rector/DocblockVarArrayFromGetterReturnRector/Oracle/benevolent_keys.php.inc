<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<array-key, string>
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
     * @var string[]
     */
    private array $items = [];

    /**
     * @return array<array-key, string>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
