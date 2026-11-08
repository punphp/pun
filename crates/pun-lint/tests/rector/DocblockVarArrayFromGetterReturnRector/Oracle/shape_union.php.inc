<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array{a: int}|array{b?: string}
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
     * @var mixed[]
     */
    private array $items = [];

    /**
     * @return array{a: int}|array{b?: string}
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
