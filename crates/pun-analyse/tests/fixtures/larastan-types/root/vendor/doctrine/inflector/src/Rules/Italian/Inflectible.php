<?php

declare(strict_types=1);

namespace Doctrine\Inflector\Rules\Italian;

use Doctrine\Inflector\Rules\Pattern;
use Doctrine\Inflector\Rules\Substitution;
use Doctrine\Inflector\Rules\Transformation;
use Doctrine\Inflector\Rules\Word;

class Inflectible
{
    /** @return iterable<Transformation> */
    public static function getSingular(): iterable
    { yield; }

    /** @return iterable<Transformation> */
    public static function getPlural(): iterable
    { yield; }

    /** @return iterable<Substitution> */
    public static function getIrregular(): iterable
    { yield; }
}
