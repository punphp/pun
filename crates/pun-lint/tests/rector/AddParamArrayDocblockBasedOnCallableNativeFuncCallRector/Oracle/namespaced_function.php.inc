<?php

final class SomeClass
{
    public function run(array $items): void
    {
        \Foo\array_map(fn (int $item) => $item, $items);
    }
}
