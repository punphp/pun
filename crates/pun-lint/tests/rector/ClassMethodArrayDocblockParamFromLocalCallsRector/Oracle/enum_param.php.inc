<?php

enum Suit
{
    case Hearts;
}

class SomeClass
{
    public function go(Suit $s): void
    {
        $this->run([$s]);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

enum Suit
{
    case Hearts;
}

class SomeClass
{
    public function go(Suit $s): void
    {
        $this->run([$s]);
    }

    /**
     * @param \Suit[] $items
     */
    private function run(array $items): void
    {
    }
}
