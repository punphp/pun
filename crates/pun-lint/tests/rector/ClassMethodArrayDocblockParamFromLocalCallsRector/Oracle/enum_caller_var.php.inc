<?php

enum Suit
{
    case Hearts;

    public function run(array $items): void
    {
    }
}

class SomeClass
{
    public function go(Suit $s): void
    {
        $s->run([1]);
    }

    private function run(array $items): void
    {
    }
}
