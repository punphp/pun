<?php

class Base
{
}

final class SomeClass extends Base
{
    /**
     * @var array<int, parent>
     */
    private array $items = [];

    public function getItems(): array
    {
        return $this->items;
    }
}
-----
<?php

class Base
{
}

final class SomeClass extends Base
{
    /**
     * @var array<int, parent>
     */
    private array $items = [];

    /**
     * @return array<int, parent>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
