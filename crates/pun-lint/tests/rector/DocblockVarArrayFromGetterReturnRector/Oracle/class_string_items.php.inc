<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<class-string<\SomeClass>>
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
     * @var class-string<\SomeClass>[]
     */
    private array $items = [];

    /**
     * @return array<class-string<\SomeClass>>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
