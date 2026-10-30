<?php

final class SomeClass
{
    private array $items = [];

    public function run($flag): void
    {
        $this->items[] = true;
        $this->items[] = false;
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @var bool[]
     */
    private array $items = [];

    public function run($flag): void
    {
        $this->items[] = true;
        $this->items[] = false;
    }
}
