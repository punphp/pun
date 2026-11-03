<?php

declare(strict_types=1);

namespace Doctrine\Inflector\Rules;

class Ruleset
{
    /** @var Transformations */
    private $regular;

    /** @var Patterns */
    private $uninflected;

    /** @var Substitutions */
    private $irregular;

    public function __construct(Transformations $regular, Patterns $uninflected, Substitutions $irregular)
    {}

    public function getRegular(): Transformations
    {}

    public function getUninflected(): Patterns
    {}

    public function getIrregular(): Substitutions
    {}
}
