<?php

namespace App;

final class Union74
{
    public function name(): string
    {
        return 'a';
    }

    public function size(): int
    {
        return 1;
    }

    public function either(bool $b)
    {
        if ($b) {
            return $this->name();
        }

        return $this->size();
    }
}
