<?php

declare(strict_types=1);

namespace Doctrine\Inflector\Rules;

use Doctrine\Inflector\WordInflector;

use function strtolower;
use function strtoupper;
use function substr;

class Substitutions implements WordInflector
{
    /** @var Substitution[] */
    private $substitutions;

    public function __construct(Substitution ...$substitutions)
    {}

    public function getFlippedSubstitutions(): Substitutions
    {}

    public function inflect(string $word): string
    {}
}
