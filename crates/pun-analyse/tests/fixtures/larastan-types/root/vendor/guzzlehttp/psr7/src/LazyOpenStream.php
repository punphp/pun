<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\StreamInterface;

/**
 * Lazily reads or writes to a file that is opened only after an IO operation
 * take place on the stream.
 */
final class LazyOpenStream implements StreamInterface
{
    use StreamDecoratorTrait;
    use NonSerializableStreamTrait;

    private string $filename;

    private string $mode;

    private StreamInterface $stream;

    /**
     * @param string $filename File to lazily open
     * @param string $mode     fopen mode to use when opening the stream
     */
    public function __construct(string $filename, string $mode)
    {}

    public function __unserialize(array $data): void
    {}

    /**
     * Creates the underlying stream lazily when required.
     */
    protected function createStream(): StreamInterface
    {}
}
