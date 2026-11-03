<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\StreamInterface;

/**
 * Compose stream implementations based on a hash of callables.
 *
 * Allows for easy testing and extension of a provided stream without needing
 * to create a concrete class for a simple extension point.
 */
#[\AllowDynamicProperties]
final class FnStream implements StreamInterface
{
    use NonSerializableStreamTrait;

    private const SLOTS = [
        '__toString', 'close', 'detach', 'rewind',
        'getSize', 'tell', 'eof', 'isSeekable', 'seek', 'isWritable', 'write',
        'isReadable', 'read', 'getContents', 'getMetadata',
    ];

    /** @var array<string, callable> */
    private array $methods;

    private bool $detached = false;

    /**
     * @param array<string, callable> $methods Hash of method name to a callable.
     */
    public function __construct(array $methods)
    {}

    /**
     * Lazily determine which methods are not implemented.
     *
     * @throws \BadMethodCallException
     */
    public function __get(string $name): void
    {}

    /**
     * The close method is called on the underlying stream only if possible.
     */
    public function __destruct()
    {}

    /**
     * An unserialize would allow the __destruct to run when the unserialized value goes out of scope.
     *
     * @throws \LogicException
     */
    public function __wakeup(): void
    {}

    public function __unserialize(array $data): void
    {}

    /**
     * Adds custom functionality to an underlying stream by intercepting
     * specific method calls.
     *
     * @param StreamInterface         $stream  Stream to decorate
     * @param array<string, callable> $methods Hash of method name to a callable
     */
    public static function decorate(StreamInterface $stream, array $methods): self
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

    private function assertAttached(): void
    {}
}
