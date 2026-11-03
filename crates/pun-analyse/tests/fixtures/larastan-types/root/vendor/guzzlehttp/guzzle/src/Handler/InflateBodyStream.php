<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\Psr7\StreamWrapper;
use GuzzleHttp\Psr7\Utils;
use Psr\Http\Message\StreamInterface;

/**
 * Inflates a gzip or deflate encoded response body as the encoded bytes arrive.
 *
 * Unlike the zlib.inflate stream filter, a read returns the bytes decoded so
 * far instead of waiting to fill the requested length, and invalid compressed
 * data always fails the read. As with libcurl, data after the compressed
 * stream, beyond up to 4 bytes after raw deflate data, fails the read that
 * follows the last decoded byte.
 *
 * @internal
 */
final class InflateBodyStream implements StreamInterface
{
    use NonSerializableTrait;

    private const CHUNK_SIZE = 8192;

    private const INVALID_DATA = 'Unable to decode the response body: invalid gzip or deflate data';

    private const TRAILING_DATA = 'Unable to decode the response body: unexpected data after the compressed stream';

    private ?StreamInterface $stream;

    private bool $deflate;

    /** @var \InflateContext|resource|null */
    private $context;

    private string $header = '';

    private string $buffer = '';

    private int $offset = 0;

    private int $position = 0;

    private bool $finished = false;

    private int $trailingAllowance = 0;

    private bool $detachable = true;

    private ?\RuntimeException $error = null;

    /**
     * @param bool $deflate Whether the body is labelled "deflate", which also
     *                      accepts raw deflate data like libcurl does.
     */
    public function __construct(StreamInterface $stream, bool $deflate = false)
    {}

    public function __toString(): string
    {}

    public function getContents(): string
    {}

    public function close(): void
    {}

    public function detach()
    {}

    public function getSize(): ?int
    {}

    public function tell(): int
    {}

    public function eof(): bool
    {}

    public function isSeekable(): bool
    {}

    public function seek(int $offset, int $whence = \SEEK_SET): void
    {}

    public function rewind(): void
    {}

    public function isWritable(): bool
    {}

    public function write(string $string): int
    {}

    public function isReadable(): bool
    {}

    public function read(int $length): string
    {}

    /**
     * @return mixed
     */
    public function getMetadata(?string $key = null)
    {}

    private function inflate(string $encoded, ?int $encoding = null): string
    {}

    private function allowTrailing(int $length): bool
    {}

    private function attached(): StreamInterface
    {}

    private function release(): void
    {}

    private function fail(string $message): \RuntimeException
    {}
}
