<?php

final class SomeClass
{
    public const BAR = 1;

    /**
     * @var array<int, 1.5|'a'|FOO>
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
     * @var array<int, 1.5|'a'|FOO>
     */
    private array $items = [];

    /**
     * @return array<int, 1.5|'a'|FOO>
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
