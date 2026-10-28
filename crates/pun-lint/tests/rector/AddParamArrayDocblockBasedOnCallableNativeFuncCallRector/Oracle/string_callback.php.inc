<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map('strlen', $items);
    }
}
