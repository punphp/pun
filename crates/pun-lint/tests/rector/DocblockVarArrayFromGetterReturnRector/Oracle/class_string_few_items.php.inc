<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, class-string<\A>|class-string<\B>>
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
     * @var array<int, class-string<\A>|class-string<\B>>
     */
    private array $items = [];

    /**
     * @return array<int, class-string<\A>|class-string<\B>>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
