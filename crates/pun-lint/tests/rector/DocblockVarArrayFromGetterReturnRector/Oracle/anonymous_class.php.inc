<?php

$object = new class {
    private array $items = [];

    /** @return int[] */
    public function getItems(): array
    {
        return $this->items;
    }
};
-----
<?php

$object = new class {
    /**
     * @var int[]
     */
    private array $items = [];

    /** @return int[] */
    public function getItems(): array
    {
        return $this->items;
    }
};
