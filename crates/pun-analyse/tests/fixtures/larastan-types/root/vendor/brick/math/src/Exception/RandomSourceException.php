<?php

declare(strict_types=1);

namespace Brick\Math\Exception;

use RuntimeException;
use Throwable;

use function get_debug_type;
use function sprintf;

/**
 * Exception thrown when random byte generation fails.
 */
final class RandomSourceException extends RuntimeException implements MathException
{
    /**
     * @internal
     *
     * @pure
     */
    public function __construct(string $message, ?Throwable $previous = null)
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function randomSourceFailure(Throwable $previous): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function invalidRandomBytesType(mixed $value): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function invalidRandomBytesLength(int $expectedLength, int $actualLength): self
    {}
}
