<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\StreamInterface;

/**
 * Stream decorator that can cache previously read bytes from a sequentially
 * read stream.
 */
final class CachingStream implements StreamInterface
{
    use StreamDecoratorTrait;
    use NonSerializableStreamTrait;

    /** @var StreamInterface Stream being wrapped */
    private StreamInterface $remoteStream;

    /** @var int Number of bytes to skip reading due to a write on the buffer */
    private int $skipReadBytes = 0;

    private StreamInterface $stream;

    private bool $detached = false;

    private bool $closed = false;

    /**
     * We will treat the buffer object as the body of the stream
     *
     * @param StreamInterface $stream Stream to cache. The cursor is assumed to be at the beginning of the stream.
     * @param StreamInterface $target Optionally specify where data is cached. Defaults to a "php://temp"
     *                                stream. A custom target is used as a random-access byte buffer to
     *                                replay the remote stream, so it must be readable, writable, and
     *                                seekable, report an accurate position and size, and store writes
     *                                losslessly. Lossy or non-seekable streams such as BufferStream and
     *                                DroppingStream are not valid targets.
     */
    public function __construct(
        StreamInterface $stream,
        ?StreamInterface $target = null
    ) {}

    public function getSize(): ?int
    {}

    public function rewind(): void
    {}

    public function seek(int $offset, int $whence = SEEK_SET): void
    {}

    public function read(int $length): string
    {}

    public function write(string $string): int
    {}

    public function eof(): bool
    {}

    public function detach()
    {}

    /**
     * Close the remote stream and any attached cache stream.
     */
    public function close(): void
    {}

    private function cacheEntireStream(): int
    {}
}
