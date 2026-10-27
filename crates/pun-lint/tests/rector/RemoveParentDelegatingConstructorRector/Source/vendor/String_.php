<?php

// Not upstream: the constructor of nikic/php-parser 5's String_ (the class extends Scalar there), which the fixtures
// extend and Rector's run loads from its vendor directory.

declare(strict_types=1);

namespace PhpParser\Node\Scalar;

class String_
{
    /**
     * @param string $value Value of the string
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(string $value, array $attributes = [])
    {
    }
}
