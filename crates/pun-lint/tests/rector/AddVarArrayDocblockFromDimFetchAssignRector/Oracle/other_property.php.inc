<?php

final class OtherProperty
{
    private array $items = [];

    private array $other = [];

    public function add(): void
    {
        $this->other[] = 5;
        $this->items[] = 'a';
    }
}
-----
<?php

final class OtherProperty
{
    /**
     * @var string[]
     */
    private array $items = [];

    /**
     * @var int[]
     */
    private array $other = [];

    public function add(): void
    {
        $this->other[] = 5;
        $this->items[] = 'a';
    }
}
