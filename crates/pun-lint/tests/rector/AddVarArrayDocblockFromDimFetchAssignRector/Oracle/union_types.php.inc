<?php

final class UnionTypes
{
    private array $items = [];

    public function add(): void
    {
        $this->items[] = 5;
        $this->items[] = 'a';
    }
}
-----
<?php

final class UnionTypes
{
    /**
     * @var int[]|string[]
     */
    private array $items = [];

    public function add(): void
    {
        $this->items[] = 5;
        $this->items[] = 'a';
    }
}
