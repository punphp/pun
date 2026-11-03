<?php

declare(strict_types=1);

namespace Brick\Math\Exception;

use RuntimeException;

use function preg_last_error_msg;
use function sprintf;

use const PHP_INT_SIZE;

/**
 * Exception thrown when the current PHP platform does not support a required feature.
 */
final class PlatformException extends RuntimeException implements MathException
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
     *
     * @codeCoverageIgnore
     */
    public static function unsupportedFloatFormat(): self
    {}

    /**
     * @internal
     *
     * @pure
     *
     * @codeCoverageIgnore
     */
    public static function unsupportedIntSize(): self
    {}

    /**
     * @internal
     *
     * @pure
     */
    public static function pcreFailure(): self
    {}
}
