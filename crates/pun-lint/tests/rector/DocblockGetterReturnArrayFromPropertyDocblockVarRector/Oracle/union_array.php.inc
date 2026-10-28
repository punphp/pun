<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var (int|string)[]
     */
    private array $items = [];

    public function getItems(): array
    {
        return $this->items;
    }
}
-----
<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var (int|string)[]
     */
    private array $items = [];

    /**
     * @return (int|string)[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
