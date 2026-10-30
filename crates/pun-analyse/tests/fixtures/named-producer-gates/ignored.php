<?php

declare(strict_types=1);

namespace NamedProducerGates;

function ignored(ParentMethod $method): void
{
    // @phpstan-ignore argument.parameterRenamedInSubtype
    $method->run(old: 'ignored');
}
