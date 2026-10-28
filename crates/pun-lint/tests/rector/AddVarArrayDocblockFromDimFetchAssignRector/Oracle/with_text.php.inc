<?php

final class WithText
{
    /**
     * The items.
     */
    private array $items = [];

    public function add(): void
    {
        $this->items[] = 5;
    }
}
-----
<?php

final class WithText
{
    /**
     * The items.
     * @var int[]
     */
    private array $items = [];

    public function add(): void
    {
        $this->items[] = 5;
    }
}
