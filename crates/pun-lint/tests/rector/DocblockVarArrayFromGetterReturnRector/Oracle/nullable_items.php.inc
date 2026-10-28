<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, ?string>
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
     * @var string[]|null[]
     */
    private array $items = [];

    /**
     * @return array<int, ?string>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
