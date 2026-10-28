<?php

class Base
{
}

final class SomeClass extends Base
{
    private array $items = [];

    /**
     * @return parent[]
     */
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
     * @var \Base[]
     */
    private array $items = [];

    /**
     * @return parent[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
