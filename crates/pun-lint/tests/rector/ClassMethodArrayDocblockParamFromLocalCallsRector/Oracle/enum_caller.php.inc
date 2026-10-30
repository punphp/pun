<?php

enum Suit
{
    case Hearts;

    public function go(): void
    {
        $this->run([1]);
    }

    private function run(array $items): void
    {
    }
}
