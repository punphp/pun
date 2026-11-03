<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Php80;

/**
 * @author Ion Bazan <ion.bazan@gmail.com>
 * @author Nico Oelgart <nicoswd@gmail.com>
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @internal
 */
final class Php80
{
    public static function fdiv(float $dividend, float $divisor): float
    {}

    public static function get_debug_type($value): string
    {}

    public static function get_resource_id($res): int
    {}

    public static function preg_last_error_msg(): string
    {}

    public static function str_contains(string $haystack, string $needle): bool
    {}

    public static function str_starts_with(string $haystack, string $needle): bool
    {}

    public static function str_ends_with(string $haystack, string $needle): bool
    {}
}
