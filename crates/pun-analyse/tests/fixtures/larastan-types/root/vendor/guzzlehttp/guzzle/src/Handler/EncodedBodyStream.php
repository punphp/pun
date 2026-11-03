<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\Psr7\StreamDecoratorTrait;
use Psr\Http\Message\StreamInterface;

/**
 * Tracks encoded response bytes beneath a content-decoding stream.
 *
 * Reads are bounded by a positive, representable Content-Length so a decoder
 * cannot consume bytes beyond the response message boundary.
 *
 * @internal
 */
final class EncodedBodyStream implements StreamInterface
{
    use NonSerializableTrait;
    use StreamDecoratorTrait;

    private StreamInterface $stream;

    private ?int $limit;

    private int $bytesRead = 0;

    public function __construct(StreamInterface $stream, string $declaredLength)
    {}

    public function eof(): bool
    {}

    public function isSeekable(): bool
    {}

    public function seek(int $offset, int $whence = \SEEK_SET): void
    {}

    public function read(int $length): string
    {}

    public function getBytesRead(): int
    {}
}
