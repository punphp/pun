<?php

declare(strict_types=1);

namespace Brick\Math\Exception;

use Brick\Math\BigInteger;
use RuntimeException;

use function sprintf;

use const PHP_INT_MAX;
use const PHP_INT_MIN;

/**
 * Exception thrown when a native integer overflow occurs.
 */
final class IntegerOverflowException extends RuntimeException implements MathException
{
    /**
     * @internal
     *
     * @pure
     */
    public function __construct(string $message)
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function integerOutOfRange(BigInteger $value): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function nativeIntegerOverflow(string $expression): self
    {}
}
