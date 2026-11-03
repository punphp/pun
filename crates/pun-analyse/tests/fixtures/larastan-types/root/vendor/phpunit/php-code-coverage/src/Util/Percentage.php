<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Util;

use function round;
use function sprintf;
use RoundingMode;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Percentage
{
    private float $fraction;
    private float $total;

    public static function fromFractionAndTotal(float $fraction, float $total): self
    {}

    private function __construct(float $fraction, float $total)
    {}

    public function asFloat(): float
    {}

    public function asString(): string
    {}

    public function asStringWithoutPercentSign(): string
    {}

    public function asFixedWidthString(): string
    {}

    /**
     * Rounding towards zero ensures that a percentage is never displayed
     * as reached (for instance "100.00%") when it is actually below that
     * threshold (for instance 99.999%).
     */
    private function asFloatRoundedTowardsZero(): float
    {}
}
