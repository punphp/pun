<?php

declare(strict_types=1);

namespace Doctrine\Inflector\Rules;

use function preg_match;

final class Pattern
{
    /** @var string */
    private $pattern;

    /** @var string */
    private $regex;

    public function __construct(string $pattern)
    {}

    public function getPattern(): string
    {}

    public function getRegex(): string
    {}

    public function matches(string $word): bool
    {}
}
