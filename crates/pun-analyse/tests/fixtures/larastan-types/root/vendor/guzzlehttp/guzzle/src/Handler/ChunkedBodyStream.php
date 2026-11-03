<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\Psr7\StreamDecoratorTrait;
use GuzzleHttp\Psr7\StreamWrapper;
use GuzzleHttp\Psr7\Utils;
use Psr\Http\Message\StreamInterface;

/**
 * Decodes the chunked transfer coding of a response body read from the
 * unfiltered transport stream, with the framing rules libcurl applies.
 *
 * Each read returns the decoded bytes that are already available, waiting on
 * the transport only when there are none, so the transport's idle timeout
 * governs every wait. The body ends as soon as the last chunk and trailer
 * section have been decoded, without another transport read. A body that ends
 * early or has malformed framing fails the read once the bytes decoded before
 * the fault have been returned, and every later read fails the same way.
 *
 * @internal
 */
final class ChunkedBodyStream implements StreamInterface
{
    use NonSerializableTrait;
    use StreamDecoratorTrait;

    private const READ_SIZE = 8192;

    private const MAX_SIZE_DIGITS = 17;

    // libcurl's trailer line and header section limits.
    private const MAX_TRAILER_LINE_LENGTH = 4093;

    private const MAX_TRAILER_SECTION_LENGTH = 307200;

    private const SIZE = 0;

    private const SIZE_LINE = 1;

    private const DATA = 2;

    private const DATA_END = 3;

    private const TRAILER = 4;

    private const TRAILER_CR = 5;

    private const FINAL_CR = 6;

    private const DONE = 7;

    private StreamInterface $stream;

    private string $buffer = '';

    private int $offset = 0;

    private int $state = self::SIZE;

    private string $digits = '';

    private int $remaining = 0;

    private string $line = '';

    private int $trailerLength = 0;

    private int $position = 0;

    private ?\RuntimeException $failure = null;

    private bool $detachable = true;

    public function __construct(StreamInterface $stream)
    {}

    public function __clone()
    {}

    public function read(int $length): string
    {}

    public function eof(): bool
    {}

    /**
     * @return mixed
     */
    public function getMetadata(?string $key = null)
    {}

    public function tell(): int
    {}

    public function getSize(): ?int
    {}

    public function isSeekable(): bool
    {}

    public function seek(int $offset, int $whence = \SEEK_SET): void
    {}

    public function isWritable(): bool
    {}

    public function write(string $string): int
    {}

    public function close(): void
    {}

    public function detach()
    {}

    /**
     * Appends what the transport has available to the buffer, waiting for at
     * least one byte.
     */
    private function fill(): bool
    {}

    /**
     * Decodes up to $length bytes from the buffer, and any framing after them,
     * so the end of the body is noticed without another transport read.
     */
    private function decode(int $length): string
    {}

    /**
     * Decodes buffered chunks in bulk, up to $length bytes, and leaves the
     * state machine at the first chunk that needs more care. Returns null when
     * it consumes nothing.
     */
    private function decodeChunks(int $length): ?string
    {}

    private function decodeSize(): void
    {}

    private function skipSizeLine(): void
    {}

    private function decodeDataEnd(): void
    {}

    private function decodeTrailer(): void
    {}
}
