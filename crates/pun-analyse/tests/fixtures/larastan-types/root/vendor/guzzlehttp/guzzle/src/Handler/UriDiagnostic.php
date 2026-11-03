<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\Psr7;
use Psr\Http\Message\UriInterface;

/**
 * @internal
 */
final class UriDiagnostic
{
    private function __construct()
    {}

    public static function redactInMessage(
        #[\SensitiveParameter]
        string $message,
        #[\SensitiveParameter]
        UriInterface $uri
    ): string {}
}
