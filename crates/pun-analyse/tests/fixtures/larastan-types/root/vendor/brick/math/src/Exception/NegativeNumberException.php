<?php

declare(strict_types=1);

namespace Brick\Math\Exception;

use RuntimeException;

/**
 * Exception thrown when attempting to perform an unsupported operation, such as a square root, on a negative number.
 */
final class NegativeNumberException extends RuntimeException implements MathException
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
    public static function squareRootOfNegativeNumber(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function nthRootOfNegativeNumber(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function toArbitraryBaseOfNegativeNumber(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function unsignedBytesOfNegativeNumber(): self
    {}
}
