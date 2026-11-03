<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\StreamInterface;

/**
 * Provides a read only stream that pumps data from a PHP callable.
 *
 * When invoking the provided callable, the PumpStream will pass the suggested
 * number of bytes to read to the callable. The callable can choose to ignore
 * this value and return fewer or more bytes than requested. Any extra data
 * returned by the callable is buffered internally until drained using the
 * read() function of the PumpStream. The callable MUST return a non-empty
 * string when data is available, or false or null when there is no more data
 * to read.
 *
 * Userland callables that declare no parameters are tolerated by PHP, but
 * length-aware callables remain the recommended formal shape.
 */
final class PumpStream implements StreamInterface
{
    use NonSerializableStreamTrait;

    /** @var callable|null */
    private $source;

    private ?int $size;

    private int $tellPos = 0;

    private array $metadata;

    private BufferStream $buffer;

    /**
     * @param (callable(): (string|false|null))|(callable(int): (string|false|null)) $source  Source of the stream data. The callable receives
     *                                                                                        the suggested number of bytes to read, may ignore
     *                                                                                        that value, and may return fewer or more bytes.
     *                                                                                        Extra bytes are buffered. The callable MUST return
     *                                                                                        a non-empty string when producing data, or false|null
     *                                                                                        on error or EOF. Userland callables that declare no
     *                                                                                        parameters are tolerated by PHP, but length-aware
     *                                                                                        callables remain the recommended formal shape.
     * @param array{size?: int, metadata?: array}                                    $options Stream options:
     *                                                                                        - metadata: Hash of metadata to use with stream.
     *                                                                                        - size: Size of the stream, if known.
     */
    public function __construct(callable $source, array $options = [])
    {}

    public function __unserialize(array $data): void
    {}

    public function __toString(): string
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

    public function rewind(): void
    {}

    public function seek(int $offset, int $whence = SEEK_SET): void
    {}

    public function isWritable(): bool
    {}

    public function write(string $string): int
    {}

    public function isReadable(): bool
    {}

    public function read(int $length): string
    {}

    public function getContents(): string
    {}

    /**
     * @return mixed
     */
    public function getMetadata(?string $key = null)
    {}

    private function pump(int $length): void
    {}
}
