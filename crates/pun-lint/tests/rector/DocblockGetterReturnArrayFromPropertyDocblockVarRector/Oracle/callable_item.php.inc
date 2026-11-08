<?php

final class SomeClass
{
    /**
     * @var array<int, callable(int): string>
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
     * @var array<int, callable(int): string>
     */
    private array $items = [];

    /**
     * @return array<int, callable(int): string>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
