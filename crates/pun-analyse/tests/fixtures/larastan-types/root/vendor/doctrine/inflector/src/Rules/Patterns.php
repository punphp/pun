<?php

declare(strict_types=1);

namespace Doctrine\Inflector\Rules;

use function array_map;
use function implode;
use function preg_match;

class Patterns
{
    /** @var string */
    private $regex;

    public function __construct(Pattern ...$patterns)
    {}

    public function matches(string $word): bool
    {}
}
