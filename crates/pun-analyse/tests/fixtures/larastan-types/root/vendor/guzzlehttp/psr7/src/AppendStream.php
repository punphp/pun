<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use GuzzleHttp\Psr7\Exception\TimeoutException;
use Psr\Http\Message\StreamInterface;

/**
 * Reads from multiple streams, one after the other.
 *
 * This is a read-only stream decorator.
 */
final class AppendStream implements StreamInterface
{
    use NonSerializableStreamTrait;

    /** @var StreamInterface[] Streams being decorated */
    private array $streams = [];

    private bool $seekable = true;

    private int $current = 0;

    private int $pos = 0;

    /**
     * @param StreamInterface[] $streams Streams to decorate. Each stream must
     *                                   be readable.
     */
    public function __construct(array $streams = [])
    {}

    public function __toString(): string
    {}

    /**
     * Add a stream to the AppendStream
     *
     * @param StreamInterface $stream Stream to append. Must be readable.
     *
     * @throws \InvalidArgumentException if the stream is not readable
     */
    public function addStream(StreamInterface $stream): void
    {}

    public function getContents(): string
    {}

    /**
     * Closes each attached stream.
     */
    public function close(): void
    {}

    /**
     * Detaches each attached stream.
     *
     * Returns null as it's not clear which underlying stream resource to return.
     */
    public function detach()
    {}

    public function tell(): int
    {}

    /**
     * Tries to calculate the size by adding the size of each stream.
     *
     * If any of the streams do not return a valid number, then the size of the
     * append stream cannot be determined and null is returned.
     */
    public function getSize(): ?int
    {}

    public function eof(): bool
    {}

    public function rewind(): void
    {}

    /**
     * Attempts to seek to the given position. Only supports SEEK_SET.
     */
    public function seek(int $offset, int $whence = SEEK_SET): void
    {}

    /**
     * Reads from all of the appended streams until the length is met or EOF.
     *
     * A timeout after some bytes were read returns those bytes, and the next
     * read waits on the same stream again.
     */
    public function read(int $length): string
    {}

    public function isReadable(): bool
    {}

    public function isWritable(): bool
    {}

    public function isSeekable(): bool
    {}

    public function write(string $string): int
    {}

    public function getMetadata(?string $key = null): ?array
    {}
}
