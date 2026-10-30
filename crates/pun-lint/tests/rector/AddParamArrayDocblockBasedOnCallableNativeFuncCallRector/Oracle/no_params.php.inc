<?php

final class SomeClass
{
    public function run(): void
    {
        array_map(fn (int $item) => $item, []);
    }
}
