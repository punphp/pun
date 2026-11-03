<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\Exception\RequestException;
use GuzzleHttp\Psr7;
use Psr\Http\Message\RequestInterface;

/**
 * Rejects request hosts that a handler could resolve differently from the host
 * the request names.
 *
 * @internal
 */
final class HostValidator
{
    private function __construct()
    {}

    /**
     * Asserts that a request names one unambiguous network host.
     *
     * A handler reparses the URI but sends the Host header as given. The URI
     * host must therefore be printable ASCII, free of percent escapes, a valid
     * RFC 3986 host, and not numeric-looking parts followed by trailing dots.
     * Every Host header value must be printable ASCII.
     *
     * @throws RequestException
     */
    public static function assertRequestHost(
        #[\SensitiveParameter]
        RequestInterface $request
    ): void {}

    /**
     * Only the URI host is reparsed for the connection. A Host header is sent
     * verbatim and may carry a port, brackets, and an RFC 6874 zone identifier.
     *
     * @throws RequestException
     */
    private static function assertUriHost(
        string $host,
        #[\SensitiveParameter]
        RequestInterface $request
    ): void {}

    /**
     * Reports whether a host has libcurl's inet_aton-style shape: one to four
     * decimal, 0-prefixed octal, or 0x-prefixed hexadecimal parts.
     *
     * Range and 32-bit overflow checks are deliberately omitted. This may
     * reject a trailing-dot spelling the transport reads as a name, but avoids
     * missing one it resolves as an address.
     */
    public static function isNumericIpv4Host(string $host): bool
    {}

    private static function isNumericIpv4Part(string $part): bool
    {}

    /**
     * @throws RequestException
     */
    private static function assertPrintableAscii(
        string $value,
        string $message,
        #[\SensitiveParameter]
        RequestInterface $request
    ): void {}

    /**
     * Escapes non-printable bytes as uppercase \xNN for safe diagnostics.
     *
     * Psr7\DiagnosticValue is not used because it preserves valid non-ASCII
     * UTF-8, including invisible characters rejected here.
     */
    private static function escape(string $value): string
    {}
}
