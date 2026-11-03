<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\StreamInterface;

/**
 * Stream decorator trait
 *
 * @property StreamInterface $stream
 */
trait StreamDecoratorTrait
{
    /**
     * @param StreamInterface $stream Stream to decorate
     */
    public function __construct(StreamInterface $stream)
    {}

    /**
     * Magic method used to create a new stream if streams are not added in
     * the constructor of a decorator (e.g., LazyOpenStream).
     */
    public function __get(string $name): StreamInterface
    {}

    public function __toString(): string
    {}

    public function getContents(): string
    {}

    /**
     * Allow decorators to implement custom methods
     *
     * @return mixed
     */
    public function __call(string $method, array $args)
    {}

    public function close(): void
    {}

    /**
     * @return mixed
     */
    public function getMetadata(?string $key = null)
    {}

    public function detach()
    {}

    public function getSize(): ?int
    {}

    public function eof(): bool
    {}

    public function tell(): int
    {}

    public function isReadable(): bool
    {}

    public function isWritable(): bool
    {}

    public function isSeekable(): bool
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
     * Implement in subclasses to dynamically create streams when requested.
     *
     * @throws \BadMethodCallException
     */
    protected function createStream(): StreamInterface
    {}
}
