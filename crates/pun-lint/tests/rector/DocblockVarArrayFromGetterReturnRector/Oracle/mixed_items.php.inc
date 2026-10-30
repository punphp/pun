<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return mixed[]
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
     * @return mixed[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
