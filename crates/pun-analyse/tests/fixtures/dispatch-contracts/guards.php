<?php

declare(strict_types=1);

namespace DispatchGuards;

interface First
{
}

interface Second
{
}

class Subject implements First
{
    public function same(): void
    {
        if (!($this instanceof First) || !($this instanceof First)) {
            $this->sameMissing();
        }
    }

    public function different(): void
    {
        if (!($this instanceof First) || !($this instanceof Second)) {
            $this->differentMissing();
        }
    }
}

function constants(): void
{
    if (defined('DISPATCH_ONE') || defined('DISPATCH_ONE')) {
        echo DISPATCH_ONE;
    }
    if (defined('DISPATCH_ONE') || defined('DISPATCH_TWO')) {
        echo DISPATCH_ONE;
    }
}
