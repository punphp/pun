<?php

$object = new class {
    private array $items = [];

    public function add(): void
    {
        $this->items[] = 5;
    }
};
-----
<?php

$object = new class {
    /**
     * @var int[]
     */
    private array $items = [];

    public function add(): void
    {
        $this->items[] = 5;
    }
};
