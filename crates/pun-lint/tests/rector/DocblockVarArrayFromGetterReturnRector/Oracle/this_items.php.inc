<?php

final class SomeClass
{
    private array $items = [];

    /**
     * @return $this[]
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
     * @var $this[]
     */
    private array $items = [];

    /**
     * @return $this[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
