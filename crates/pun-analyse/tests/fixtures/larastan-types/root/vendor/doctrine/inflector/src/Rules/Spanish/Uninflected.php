<?php

declare(strict_types=1);

namespace Doctrine\Inflector\Rules\Spanish;

use Doctrine\Inflector\Rules\Pattern;

final class Uninflected
{
    /** @return Pattern[] */
    public static function getSingular(): iterable
    { yield; }

    /** @return Pattern[] */
    public static function getPlural(): iterable
    { yield; }

    /** @return Pattern[] */
    private static function getDefault(): iterable
    { yield; }
}
