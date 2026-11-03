<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\Psr7\StreamDecoratorTrait;
use Psr\Http\Message\StreamInterface;

/**
 * Ends a streamed response body at its declared Content-Length.
 *
 * Reads never consume bytes beyond the response message, and the body reaches
 * EOF without waiting for the server to close the connection. A transport that
 * ends before the declared length fails the read instead of looking like a
 * complete body.
 *
 * @internal
 */
final class DeclaredLengthStream implements StreamInterface
{
    use NonSerializableTrait;
    use StreamDecoratorTrait;

    private StreamInterface $stream;

    private int $remaining;

    private ?\RuntimeException $failure = null;

    public function __construct(StreamInterface $stream, int $declaredLength)
    {}

    public function __clone()
    {}

    public function eof(): bool
    {}

    public function isSeekable(): bool
    {}

    public function seek(int $offset, int $whence = \SEEK_SET): void
    {}

    /**
     * @return mixed
     */
    public function getMetadata(?string $key = null)
    {}

    public function read(int $length): string
    {}
}
