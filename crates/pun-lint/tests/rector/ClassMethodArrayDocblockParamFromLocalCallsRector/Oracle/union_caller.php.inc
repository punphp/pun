<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $either = rand(0, 1) ? $this : new \ArrayObject();
        $either->run(['a']);
    }

    private function run(array $items): void
    {
    }
}
