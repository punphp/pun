<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        self::run(['a']);
        $f = self::run(...);
    }

    private static function run(array $items): void
    {
    }
}
