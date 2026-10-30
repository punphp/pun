<?php

declare(strict_types=1);

namespace DispatchAnonymous;

#[\Attribute(\Attribute::TARGET_CLASS)]
class Marker
{
    public function __construct(public int $value)
    {
    }
}

function values(): array
{
    return [
        new #[Marker(1)] readonly class {
        },
        new #[Marker(MISSING_ATTRIBUTE_VALUE)] readonly class {
        },
        new #[Marker('wrong')] readonly class {
        },
    ];
}
