<?php

declare(strict_types=1);

namespace GuzzleHttp;

use GuzzleHttp\Exception\InvalidArgumentException;
use Psr\Http\Message\UriInterface;

/**
 * @internal
 */
final class Idn
{
    private function __construct()
    {}

    /**
     * @param mixed $value
     */
    public static function normalizeConversionOption($value): ?int
    {}

    /**
     * @throws InvalidArgumentException
     */
    public static function convertUri(UriInterface $uri, int $options = 0): UriInterface
    {}

    /**
     * @return string|false
     */
    private static function idnToAsci(string $domain, int $options, ?array &$info = [])
    {}
}
