<?php

declare(strict_types=1);

namespace PropertyHolderFallback;

final class Holder
{
    public int $written = 1;
    private ?int $narrow = 1;

    public function run(): void
    {
        return;
        $this->written = 2;
    }

    public function read(): ?int
    {
        return $this->narrow;
    }
}
