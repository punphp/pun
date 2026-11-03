<?php

declare(strict_types=1);

namespace Brick\Math\Internal;

use Brick\Math\RoundingMode;

use function ltrim;
use function rtrim;
use function str_pad;
use function str_repeat;
use function strlen;
use function substr;

use const STR_PAD_LEFT;

/**
 * Shared helper for decimal operations.
 *
 * @internal
 */
final class DecimalHelper
{
    /**
     * @codeCoverageIgnore
     */
    private function __construct()
    {}

    /**
     * Computes the scale needed to represent the exact decimal result of a reduced fraction.
     *
     * Returns null if the denominator has prime factors other than 2 or 5.
     *
     * @param string $denominator The denominator of the reduced fraction. Must be strictly positive.
     *
     * @return non-negative-int|null
     *
     * @pure
     */
    public static function computeScaleFromReducedFractionDenominator(string $denominator): ?int
    {}

    /**
     * Scales an unscaled decimal value to the requested scale.
     *
     * Returns null when rounding is necessary and the rounding mode is Unnecessary.
     *
     * @param string       $value        The unscaled value.
     * @param int          $currentScale The current scale.
     * @param int          $targetScale  The target scale.
     * @param RoundingMode $roundingMode The rounding mode.
     *
     * @return string|null The unscaled value at the target scale, or null if RoundingMode::Unnecessary is used and rounding is necessary.
     *
     * @pure
     */
    public static function scale(string $value, int $currentScale, int $targetScale, RoundingMode $roundingMode): ?string
    {}

    /**
     * Adds leading zeros if necessary to represent the full decimal number.
     *
     * @param string $value The unscaled value.
     * @param int    $scale The current scale.
     *
     * @pure
     */
    public static function padUnscaledValue(string $value, int $scale): string
    {}

    /**
     * Tries to scale exactly without rounding, returning null when rounding would be required.
     *
     * @param string $value        The unscaled value.
     * @param int    $currentScale The current scale.
     * @param int    $targetScale  The target scale.
     *
     * @return string|null The unscaled value at the target scale, or null if rounding would be required.
     *
     * @pure
     */
    public static function tryScaleExactly(string $value, int $currentScale, int $targetScale): ?string
    {}
}
