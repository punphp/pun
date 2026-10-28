<?php

namespace App;

final class Item {}

final class ClassItems
{
    /**
     * @var Item[]
     */
    private array $items = [];

    public function getItems(): array
    {
        return $this->items;
    }
}
-----
<?php

namespace App;

final class Item {}

final class ClassItems
{
    /**
     * @var Item[]
     */
    private array $items = [];

    /**
     * @return Item[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
