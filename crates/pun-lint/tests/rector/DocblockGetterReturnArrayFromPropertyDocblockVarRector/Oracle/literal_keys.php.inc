<?php

final class SomeClass
{
    /**
     * @var array<'a'|'b', int>
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
     * @var array<'a'|'b', int>
     */
    private array $items = [];

    /**
     * @return array<'a'|'b', int>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
