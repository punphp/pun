<?php

$object = new class (['a']) {
    private array $items;

    /**
     * @param string[] $items
     */
    public function __construct(array $items)
    {
        $this->items = $items;
    }
};
-----
<?php

$object = new class (['a']) {
    /**
     * @var string[]
     */
    private array $items;

    /**
     * @param string[] $items
     */
    public function __construct(array $items)
    {
        $this->items = $items;
    }
};
