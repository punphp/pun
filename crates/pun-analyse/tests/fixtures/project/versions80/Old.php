<?php

namespace Versions80;

interface HasName
{
    public string $name { get; }
}

final class Features
{
    final public string $final = '';

    public string $hooked {
        get => 'hooked';
    }

    public function __construct(public int $promoted)
    {
    }
}
