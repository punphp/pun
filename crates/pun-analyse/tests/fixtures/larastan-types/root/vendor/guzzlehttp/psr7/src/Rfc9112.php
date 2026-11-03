<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

/**
 * @internal
 */
final class Rfc9112
{
    /**
     * An HTTP protocol version for use in a regular expression.
     */
    public const PROTOCOL_VERSION_PATTERN = '\d+(?:\.\d+)?';

    /**
     * The request-target bytes accepted by the HTTP/1 start-line grammar for
     * use in a regular expression.
     */
    public const REQUEST_TARGET_PATTERN = '[^\x00-\x20\x7F]+';

    private function __construct()
    {}

    /**
     * Header related regular expressions (based on amphp/http package)
     *
     * Note: header delimiter (\r\n) is modified to \r?\n to accept line feed only delimiters for BC reasons.
     *
     * @see https://github.com/amphp/http/blob/v1.0.1/src/Rfc7230.php#L12-L15
     *
     * @license https://github.com/amphp/http/blob/v1.0.1/LICENSE
     */
    public const HEADER_REGEX = "(^([^()<>@,;:\\\"/[\]?={}\x01-\x20\x7F]++):[ \t]*+((?:[ \t]*+[\x21-\x7E\x80-\xFF]++)*+)[ \t]*+\r?\n)m";
    public const HEADER_FOLD_REGEX = "(\r?\n[ \t]++)";

    public static function isValidProtocolVersion(string $version): bool
    {}

    public static function isValidRequestTarget(string $target): bool
    {}

    public static function isValidReasonPhrase(string $reasonPhrase): bool
    {}

    /**
     * @return array{0: string, 1: int|null}|null
     */
    public static function parseHostHeader(string $authority): ?array
    {}

    public static function isAbsoluteFormRequestTarget(string $target): bool
    {}

    public static function isAsteriskFormRequestTarget(string $method, string $target): bool
    {}

    public static function isConnectAuthorityFormRequestTarget(string $method, string $target): bool
    {}

    public static function parsePort(string $port): ?int
    {}
}
