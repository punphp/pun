<?php

namespace App;

final class Dynamic
{
    private $name;
    private $dynamic;

    public function run(string $name, string $key): void
    {
        $this->name = $name;
        $this->{$key} = 1;
        $this->dynamic = 1;
    }
}
