<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<string, int|string>
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
     * @var array<string, int|string>
     */
    private array $items = [];

    /**
     * @return array<string, int|string>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
