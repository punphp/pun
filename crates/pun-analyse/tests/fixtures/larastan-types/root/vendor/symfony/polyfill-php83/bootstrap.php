<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

use Symfony\Polyfill\Php83 as p;

if (\PHP_VERSION_ID >= 80300) {
    return;
}

if (!function_exists('json_validate')) {
    function json_validate(string $json, int $depth = 512, int $flags = 0): bool {}
}

if (!function_exists('stream_context_set_options')) {
    function stream_context_set_options($context, array $options): bool {}
}

if (!function_exists('str_increment')) {
    function str_increment(string $string): string {}
}

if (!function_exists('str_decrement')) {
    function str_decrement(string $string): string {}
}

if (\PHP_VERSION_ID < 80000) {
    require __DIR__.'/bootstrap72.php';
}

if (extension_loaded('mbstring')) {
    if (!function_exists('mb_str_pad')) {
        function mb_str_pad(?string $string, ?int $length, ?string $pad_string = ' ', ?int $pad_type = STR_PAD_RIGHT, ?string $encoding = null): string {}
    }
}

if (\PHP_VERSION_ID >= 80100) {
    return require __DIR__.'/bootstrap81.php';
}

if (!function_exists('ldap_exop_sync') && function_exists('ldap_exop')) {
    function ldap_exop_sync($ldap, string $request_oid, ?string $request_data = null, ?array $controls = null, &$response_data = null, &$response_oid = null): bool {}
}

if (!function_exists('ldap_connect_wallet') && function_exists('ldap_connect')) {
    function ldap_connect_wallet(?string $uri, string $wallet, string $password, int $auth_mode = \GSLC_SSL_NO_AUTH) {}
}
