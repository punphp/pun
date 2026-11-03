<?php

declare(strict_types=1);

namespace Doctrine\Inflector\Rules\Italian;

use Doctrine\Inflector\Rules\Pattern;

final class Uninflected
{
    /** @return iterable<Pattern> */
    public static function getSingular(): iterable
    { yield; }

    /** @return iterable<Pattern> */
    public static function getPlural(): iterable
    { yield; }

    /** @return iterable<Pattern> */
    private static function getDefault(): iterable
    { yield; }
}
