<?php

declare(strict_types=1);

namespace Doctrine\Inflector\Rules\Spanish;

use Doctrine\Inflector\Rules\Pattern;
use Doctrine\Inflector\Rules\Substitution;
use Doctrine\Inflector\Rules\Transformation;
use Doctrine\Inflector\Rules\Word;

class Inflectible
{
    /** @return Transformation[] */
    public static function getSingular(): iterable
    { yield; }

    /** @return Transformation[] */
    public static function getPlural(): iterable
    { yield; }

    /** @return Substitution[] */
    public static function getIrregular(): iterable
    { yield; }
}
