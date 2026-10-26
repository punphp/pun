<?php

namespace App;

final readonly class Value
{
    public function __construct(public string $value) {}

    public function withValue(string $value): self
    {
        return clone($this, ['value' => $value]);
    }
}

$exit = exit('done');
$die = die('done');
