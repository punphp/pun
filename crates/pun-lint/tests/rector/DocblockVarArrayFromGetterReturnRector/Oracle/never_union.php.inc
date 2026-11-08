<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<string, never>|array<int, int>
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
     * @var array<int|string, mixed>
     */
    private array $items = [];

    /**
     * @return array<string, never>|array<int, int>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
