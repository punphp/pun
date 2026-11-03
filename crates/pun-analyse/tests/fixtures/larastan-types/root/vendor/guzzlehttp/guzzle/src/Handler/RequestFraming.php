<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\Exception\RequestException;
use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\Psr7;
use GuzzleHttp\Psr7\Exception\TimeoutException;
use Psr\Http\Message\RequestInterface;

/**
 * Validates and normalizes request framing before a handler performs I/O.
 *
 * Body size describes the bytes available under handler rewind semantics.
 * Content-Length is the selected transport boundary and may remain unknown.
 *
 * @internal
 */
final class RequestFraming
{
    use NonSerializableTrait;

    public RequestInterface $request;

    /**
     * Number of bytes available after applying handler rewind semantics, if
     * known.
     */
    public ?int $bodySize;

    /** Canonical Content-Length boundary selected for transport, if any. */
    public ?int $contentLength;

    private function __construct(RequestInterface $request, ?int $bodySize, ?int $contentLength)
    {}

    /**
     * Finalizes request framing for a handler dispatch.
     *
     * When $sendBody is false, body-dependent validation and size probing are
     * skipped. Content-Length is still canonicalized and Transfer-Encoding is
     * removed.
     *
     * @throws RequestException when framing or body metadata is unsafe
     */
    public static function analyze(
        #[\SensitiveParameter]
        RequestInterface $request,
        bool $sendBody = true
    ): self {}

    /**
     * Returns the number of bytes available for dispatch, or null when unknown.
     *
     * Seekable bodies use their total size because handlers rewind them.
     * Positioned non-seekable bodies use the total size minus their current
     * position.
     *
     * @throws RequestException when body metadata cannot be read safely
     */
    public static function bodySize(
        #[\SensitiveParameter]
        RequestInterface $request
    ): ?int {}

    /**
     * Materializes the body, stopping at the selected Content-Length when
     * present.
     *
     * @throws RequestException when the body cannot be fully rewound or read
     */
    public function materialize(): string
    {}

    private static function bodyException(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        \Exception $exception,
        string $timeoutMessage,
        string $fallbackMessage
    ): RequestException {}
}
