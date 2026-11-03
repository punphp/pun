<?php

declare(strict_types=1);

namespace GuzzleHttp;

/**
 * Reads configuration from the process environment.
 *
 * Intentionally separate from Handler\ProxyEnv, which has different
 * lookup semantics.
 *
 * @internal
 */
final class Env
{
    private function __construct()
    {}

    public static function get(string $name): ?string
    {}
}
