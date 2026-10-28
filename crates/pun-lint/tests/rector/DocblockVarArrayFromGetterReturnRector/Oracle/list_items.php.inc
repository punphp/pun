<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return list<string>
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
     * @var list<string>
     */
    private array $items = [];

    /**
     * @return list<string>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
