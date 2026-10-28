<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return array<int, object>
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
     * @var object[]
     */
    private array $items = [];

    /**
     * @return array<int, object>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
