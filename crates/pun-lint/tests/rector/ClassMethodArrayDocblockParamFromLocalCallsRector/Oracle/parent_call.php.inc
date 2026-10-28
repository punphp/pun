<?php

class SomeClass extends \ArrayObject
{
    public function go(self $other, string $name): void
    {
        parent::run(['a']);
    }

    private function run(array $items): void
    {
    }
}
