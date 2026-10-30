<?php

enum Suit
{
    case Hearts;
}

class SomeClass
{
    public function go(): void
    {
        $this->run([Suit::Hearts]);
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
    public function go(): void
    {
        $this->run([Suit::Hearts]);
    }

    /**
     * @param Suit::Hearts[] $items
     */
    private function run(array $items): void
    {
    }
}
