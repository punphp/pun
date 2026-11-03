<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Php84;

/**
 * @author Ayesh Karunaratne <ayesh@aye.sh>
 * @author Pierre Ambroise <pierre27.ambroise@gmail.com>
 *
 * @internal
 */
final class Php84
{
    private const BCMATH_WELL_FORMED = '/^[+-]?[0-9]*+(?:\.[0-9]*+)?$/D';

    /** @return string|false */
    public static function mb_ucfirst(string $string, ?string $encoding = null)
    {}

    /** @return string|false */
    public static function mb_lcfirst(string $string, ?string $encoding = null)
    {}

    public static function array_find(array $array, callable $callback)
    {}

    public static function array_find_key(array $array, callable $callback)
    {}

    public static function array_any(array $array, callable $callback): bool
    {}

    public static function array_all(array $array, callable $callback): bool
    {}

    public static function fpow(float $num, float $exponent): float
    {}

    /** @return string|false */
    public static function mb_trim(string $string, ?string $characters = null, ?string $encoding = null)
    {}

    /** @return string|false */
    public static function mb_ltrim(string $string, ?string $characters = null, ?string $encoding = null)
    {}

    /** @return string|false */
    public static function mb_rtrim(string $string, ?string $characters = null, ?string $encoding = null)
    {}

    /** @return string|false */
    private static function mb_internal_trim(string $regex, string $string, ?string $characters, ?string $encoding, string $function)
    {}

    /**
     * Trims ill-formed UTF-8 like mbstring: each maximal subpart of an ill-formed
     * sequence counts as one character, and the string is returned as is when
     * nothing is trimmed, else re-encoded with the substitute character.
     */
    private static function mb_trim_invalid_utf8(string $string, ?string $characters, string $function): string
    {}

    public static function grapheme_str_split(string $string, int $length)
    {}

    public static function bcceil(string $num): string
    {}

    public static function bcdivmod(string $num1, string $num2, ?int $scale = null): ?array
    {}

    public static function bcfloor(string $num): string
    {}

    /**
     * @param \RoundingMode|\RoundingMode::* $mode
     */
    public static function bcround(string $num, int $precision = 0, $mode = \RoundingMode::HalfAwayFromZero): string
    {}

    private static function roundIntegerPart(string $intPart, string $fracPart, int $sign, $mode): string
    {}

    private static function formatRoundedDigits(string $roundedInt, int $precision): string
    {}

    private static function incrementDigits(string $digits): string
    {}

    private static function trimLeadingZeros(string $digits): string
    {}

    private static function lastDigit(string $digits): int
    {}

    /**
     * Splits a string that is not valid UTF-8 into grapheme clusters.
     *
     * The intl extension keeps the ill-formed bytes and breaks around them the
     * way it would around the U+FFFD each maximal subpart stands for. Standing
     * one in for real makes the cluster regexp applicable, then each cluster is
     * mapped back to the bytes it was made of.
     */
    private static function splitIllFormed(string $string, string $regex): array
    {}
}
