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
     * @var array<string, mixed>|int[]
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
