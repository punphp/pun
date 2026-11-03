<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

use Symfony\Polyfill\Php80 as p;

if (\PHP_VERSION_ID >= 80000) {
    return;
}

if (!defined('FILTER_VALIDATE_BOOL') && defined('FILTER_VALIDATE_BOOLEAN')) {
    define('FILTER_VALIDATE_BOOL', \FILTER_VALIDATE_BOOLEAN);
}

if (!function_exists('fdiv')) {
    function fdiv(float $num1, float $num2): float {}
}
if (!function_exists('preg_last_error_msg')) {
    function preg_last_error_msg(): string {}
}
if (!function_exists('str_contains')) {
    function str_contains(?string $haystack, ?string $needle): bool {}
}
if (!function_exists('str_starts_with')) {
    function str_starts_with(?string $haystack, ?string $needle): bool {}
}
if (!function_exists('str_ends_with')) {
    function str_ends_with(?string $haystack, ?string $needle): bool {}
}
if (!function_exists('get_debug_type')) {
    function get_debug_type($value): string {}
}
if (!function_exists('get_resource_id')) {
    function get_resource_id($resource): int {}
}
