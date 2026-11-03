<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Php86;

/**
 * @author kylekatarnls <kylekatarnls@gmail.com>
 *
 * @internal
 */
final class Php86
{
    /**
     * @template Value
     * @template Minimum
     * @template Maximum
     *
     * @param Value   $value
     * @param Minimum $min
     * @param Maximum $max
     *
     * @return Value|Minimum|Maximum
     */
    public static function clamp($value, $min, $max)
    {}

    private static function throwValueErrorIfAvailable(string $message): void
    {}

    public static function grapheme_strrev(string $string)
    {}
}
