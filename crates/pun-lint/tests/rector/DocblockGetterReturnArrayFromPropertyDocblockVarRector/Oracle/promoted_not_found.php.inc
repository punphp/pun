<?php

final class PromotedNotFound
{
    public function __construct(
        /** @var int[] */
        private array $items,
        array $other = [],
    ) {
    }

    public function getOther(): array
    {
        return $this->other;
    }

    public function getItems(): array
    {
        return $this->items;
    }
}
-----
<?php

final class PromotedNotFound
{
    public function __construct(
        /** @var int[] */
        private array $items,
        array $other = [],
    ) {
    }

    public function getOther(): array
    {
        return $this->other;
    }

    /**
     * @return int[]
     */
    public function getItems(): array
    {
        return $this->items;
    }
}
