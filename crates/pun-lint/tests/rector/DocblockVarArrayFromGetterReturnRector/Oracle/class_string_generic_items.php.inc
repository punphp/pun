<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, class-string<\ArrayObject<int, string>>>
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
     * @var class-string<ArrayObject<int, string>>[]
     */
    private array $items = [];

    /**
     * @return array<int, class-string<\ArrayObject<int, string>>>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
