<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

use Symfony\Polyfill\Intl\Grapheme as p;

if (\PHP_VERSION_ID >= 80000) {
    return require __DIR__.'/bootstrap80.php';
}

if (!class_exists('ValueError', false)) {
    class ValueError extends Error
    {
    }
}

if (!defined('GRAPHEME_EXTR_COUNT')) {
    define('GRAPHEME_EXTR_COUNT', 0);
}
if (!defined('GRAPHEME_EXTR_MAXBYTES')) {
    define('GRAPHEME_EXTR_MAXBYTES', 1);
}
if (!defined('GRAPHEME_EXTR_MAXCHARS')) {
    define('GRAPHEME_EXTR_MAXCHARS', 2);
}

if (!function_exists('grapheme_extract')) {
    function grapheme_extract($haystack, $size, $type = 0, $start = 0, &$next = 0) {}
}
if (!function_exists('grapheme_stripos')) {
    function grapheme_stripos($haystack, $needle, $offset = 0) {}
}
if (!function_exists('grapheme_stristr')) {
    function grapheme_stristr($haystack, $needle, $beforeNeedle = false) {}
}
if (!function_exists('grapheme_strlen')) {
    function grapheme_strlen($input) {}
}
if (!function_exists('grapheme_strpos')) {
    function grapheme_strpos($haystack, $needle, $offset = 0) {}
}
if (!function_exists('grapheme_strripos')) {
    function grapheme_strripos($haystack, $needle, $offset = 0) {}
}
if (!function_exists('grapheme_strrpos')) {
    function grapheme_strrpos($haystack, $needle, $offset = 0) {}
}
if (!function_exists('grapheme_strstr')) {
    function grapheme_strstr($haystack, $needle, $beforeNeedle = false) {}
}
if (!function_exists('grapheme_substr')) {
    function grapheme_substr($string, $offset, $length = null) {}
}
if (!function_exists('grapheme_str_split')) {
    function grapheme_str_split(string $string, int $length = 1) {}
}
if (!function_exists('grapheme_levenshtein')) {
    function grapheme_levenshtein(string $string1, string $string2, ?int $insertion_cost = 1, ?int $replacement_cost = 1, ?int $deletion_cost = 1, string $locale = '') {}
}
if (!function_exists('grapheme_strrev')) {
    function grapheme_strrev(string $string) {}
}
