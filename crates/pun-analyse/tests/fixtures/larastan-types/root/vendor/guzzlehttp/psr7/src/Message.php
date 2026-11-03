<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\MessageInterface;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Message\UriInterface;

final class Message
{
    private const DEFAULT_BODY_SUMMARY_TRUNCATE_AT = 120;

    private function __construct()
    {}

    /**
     * Returns the string representation of an HTTP message.
     *
     * @param MessageInterface $message Message to convert to a string.
     */
    public static function toString(MessageInterface $message): string
    {}

    private static function hostHeaderFromUri(UriInterface $uri): string
    {}

    /**
     * Get a short summary of the message body.
     *
     * Will return `null` if the response is not printable.
     *
     * Reads seekable bodies from the beginning and restores the original cursor
     * position before returning. Pass `null` for `$truncateAt` to use the
     * default summary length.
     *
     * @param MessageInterface $message    The message to get the body summary
     * @param int|null         $truncateAt Maximum allowed size of the summary
     */
    public static function bodySummary(MessageInterface $message, ?int $truncateAt = null): ?string
    {}

    /**
     * Trims a partial UTF-8 character from the end of a truncated string.
     */
    private static function trimTrailingIncompleteUtf8Character(string $summary, string $lookahead): string
    {}

    /**
     * Attempts to rewind a message body and throws an exception on failure.
     *
     * The body of the message will only be rewound if a call to `tell()`
     * returns a value other than `0`.
     *
     * @param MessageInterface $message Message to rewind
     *
     * @throws \RuntimeException
     */
    public static function rewindBody(MessageInterface $message): void
    {}

    /**
     * Parses an HTTP message into an associative array.
     *
     * The array contains the `start-line` key containing the start line of the
     * message, `headers` key containing an associative array of header array
     * values, and a `body` key containing the body of the message.
     *
     * @param string $message HTTP request or response to parse.
     */
    public static function parseMessage(string $message): array
    {}

    /**
     * Constructs a URI for an HTTP request message.
     *
     * The URI is composed from the start-line path and the `Host` header, using
     * `https` when the host's port is `443` and `http` otherwise. Without a
     * `Host` header, only the path is returned, with extra leading slashes
     * collapsed so an origin-form target cannot be parsed as a network-path
     * reference with its own authority. An `InvalidArgumentException` is thrown
     * when the `Host` header is invalid.
     *
     * @param string $path    Path from the start-line
     * @param array  $headers Array of headers (each value an array).
     */
    public static function parseRequestUri(string $path, array $headers): string
    {}

    /**
     * Parses a request message string into a request object.
     *
     * The request-target must be in origin form, absolute form (without a
     * userinfo component), authority form (`CONNECT`), or asterisk form
     * (`OPTIONS`), and any `Host` header must be a single valid value;
     * otherwise an `InvalidArgumentException` is thrown. Non-origin-form
     * targets are preserved on the returned request via `withRequestTarget()`.
     *
     * @param string $message Request message string.
     */
    public static function parseRequest(string $message): RequestInterface
    {}

    /**
     * Parses a response message string into a response object.
     *
     * @param string $message Response message string.
     */
    public static function parseResponse(string $message): ResponseInterface
    {}
}
