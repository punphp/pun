<?php

final class SomeClass
{
    private array $items = [];

    public function run($flag): void
    {
        $this->items[] = 1.5;
        $this->items[] = 2.5;
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @var float[]
     */
    private array $items = [];

    public function run($flag): void
    {
        $this->items[] = 1.5;
        $this->items[] = 2.5;
    }
}
