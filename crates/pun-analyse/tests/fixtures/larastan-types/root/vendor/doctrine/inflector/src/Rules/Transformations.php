<?php

declare(strict_types=1);

namespace Doctrine\Inflector\Rules;

use Doctrine\Inflector\WordInflector;

class Transformations implements WordInflector
{
    /** @var Transformation[] */
    private $transformations;

    public function __construct(Transformation ...$transformations)
    {}

    public function inflect(string $word): string
    {}
}
