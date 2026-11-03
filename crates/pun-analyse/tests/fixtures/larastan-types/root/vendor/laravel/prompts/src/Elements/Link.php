<?php

namespace Laravel\Prompts\Elements;

class Link implements ElementContract
{
    public readonly string $label;

    public function __construct(
        public readonly string $url,
        ?string $label = null,
        public readonly bool $underline = true,
    ) {}

    public function __toString(): string
    {}
}
