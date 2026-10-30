<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var array<int, callable(int $a, string &...$b=): (int|string)>
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
     * @var array<int, callable(int $a, string &...$b=): (int|string)>
     */
    private array $items = [];

    /**
     * @return array<int, callable(int $a, string &...$b=):((int|string))>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
