<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return list<int>|list<string>
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
     * @var list<int>|list<string>
     */
    private array $items = [];

    /**
     * @return list<int>|list<string>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
