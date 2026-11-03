<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use GuzzleHttp\Psr7\Exception\TimeoutException;
use Psr\Http\Message\StreamInterface;

/**
 * PHP stream implementation.
 */
class Stream implements StreamInterface
{
    use NonSerializableStreamTrait;

    /** @var resource */
    private $stream;
    private ?int $size = null;
    private bool $seekable;
    private bool $readable;
    private bool $writable;
    private ?string $uri = null;
    /** @var mixed[] */
    private array $customMetadata;

    /**
     * This constructor accepts an associative array of options.
     *
     * - size: (int) If a read stream would otherwise have an indeterminate
     *   size, but the size is known due to foreknowledge, then you can
     *   provide that size, in bytes.
     * - metadata: (array) Any additional metadata to return when the metadata
     *   of the stream is accessed.
     *
     * @param resource                            $stream  Stream resource to wrap.
     * @param array{size?: int, metadata?: array} $options Associative array of options.
     *
     * @throws \InvalidArgumentException if the stream is not a stream resource
     */
    public function __construct($stream, array $options = [])
    {}

    /**
     * Closes the stream when the destructed
     */
    public function __destruct()
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

    public function isReadable(): bool
    {}

    public function isWritable(): bool
    {}

    public function isSeekable(): bool
    {}

    public function eof(): bool
    {}

    public function tell(): int
    {}

    public function rewind(): void
    {}

    public function seek(int $offset, int $whence = SEEK_SET): void
    {}

    public function read(int $length): string
    {}

    public function write(string $string): int
    {}

    /**
     * @return mixed
     */
    public function getMetadata(?string $key = null)
    {}

    /**
     * @see https://www.php.net/manual/en/function.fopen.php
     * @see https://www.php.net/manual/en/function.gzopen.php
     */
    private static function isReadableMode(string $mode): bool
    {}

    /**
     * @see https://www.php.net/manual/en/function.fopen.php
     * @see https://www.php.net/manual/en/function.gzopen.php
     */
    private static function isWritableMode(string $mode): bool
    {}

    private function timedOut(): bool
    {}

    private function writeTimedOut(): bool
    {}
}
