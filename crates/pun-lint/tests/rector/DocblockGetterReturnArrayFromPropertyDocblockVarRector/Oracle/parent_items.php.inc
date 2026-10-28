<?php

final class SomeClass extends \ArrayObject
{
    /**
     * @var parent[]
     */
    private array $items = [];

    public function getItems(): array
    {
        return $this->items;
    }
}
-----
<?php

final class SomeClass extends \ArrayObject
{
    /**
     * @var parent[]
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
