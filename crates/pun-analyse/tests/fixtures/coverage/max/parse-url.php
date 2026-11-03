<?php declare(strict_types=1);

namespace MaxParseUrl;

// ParseUrlFunctionDynamicReturnTypeExtension, and isset() on a loop variable's constant keys
function isValidUrl(string $token): bool
{
    $components = parse_url($token);
    if ($components === false) {
        return false;
    }
    foreach (['path', 'query', 'fragment'] as $part) {
        if (isset($components[$part]) && preg_match('/[\s<>]/', $components[$part])) {
            return false;
        }
        if (isset($components[$part])) {
            \PHPStan\dumpType($components[$part]);
        }
    }
    if (isset($components['port'])) {
        \PHPStan\dumpType($components['port']);
    }
    \PHPStan\dumpType(parse_url($token, PHP_URL_PORT));
    \PHPStan\dumpType(parse_url($token, PHP_URL_HOST));
    \PHPStan\dumpType(parse_url(strtolower($token), PHP_URL_PATH));
    \PHPStan\dumpType(parse_url($token, 42));

    return true;
}
