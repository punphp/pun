<?php

final class SomeClass
{
    private array $items = [];

    public function run(): void
    {
        $this->{'items'}[] = 1;
    }
}
