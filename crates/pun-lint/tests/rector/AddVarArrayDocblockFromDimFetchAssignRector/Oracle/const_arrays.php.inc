<?php

final class SomeClass
{
    private array $items = [];

    public function run($flag): void
    {
        $this->items[] = [1];
        $this->items[] = [2];
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @var int[][]
     */
    private array $items = [];

    public function run($flag): void
    {
        $this->items[] = [1];
        $this->items[] = [2];
    }
}
