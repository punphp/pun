<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use GuzzleHttp\Psr7\Exception\TimeoutException;
use Psr\Http\Message\StreamInterface;

/**
 * @internal
 */
final class StreamTimeout
{
    private function __construct()
    {}

    public static function read(StreamInterface $stream, int $length, string $timeoutMessage): string
    {}

    public static function throwIfReadTimedOut(
        StreamInterface $stream,
        string $message,
        ?\Throwable $previous = null
    ): void {}

    public static function throwIfWriteTimedOut(StreamInterface $stream, ?\Throwable $previous = null): void
    {}

    public static function isReadTimedOut(StreamInterface $stream): bool
    {}

    public static function isWriteTimedOut(StreamInterface $stream): bool
    {}

    /**
     * @param resource $resource
     */
    public static function isResourceReadTimedOut($resource): bool
    {}

    /**
     * @param resource $resource
     */
    public static function isResourceWriteTimedOut($resource): bool
    {}
}
