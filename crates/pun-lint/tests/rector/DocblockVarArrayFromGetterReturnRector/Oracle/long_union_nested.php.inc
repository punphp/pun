<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<string, array<int, \SomeVeryLongClassNameNumberOne>>|array<string, array<int, string>>|array<string, array<int, float>>
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
     * @var array<array<string, mixed[]>, mixed>
     */
    private array $items = [];

    /**
     * @return array<string, array<int, \SomeVeryLongClassNameNumberOne>>|array<string, array<int, string>>|array<string, array<int, float>>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
