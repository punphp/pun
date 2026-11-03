<?php declare(strict_types=1);
/*
 * This file is part of sebastian/comparator.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Comparator;

use function abs;
use function assert;
use function is_float;
use function is_infinite;
use function is_nan;
use function is_numeric;
use function is_string;
use function sprintf;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for sebastian/comparator
 *
 * @internal This class is not covered by the backward compatibility promise for sebastian/comparator
 */
final class NumericComparator extends ScalarComparator
{
    public function accepts(mixed $expected, mixed $actual): bool
    {}

    /**
     * @throws ComparisonFailure
     */
    public function assertEquals(mixed $expected, mixed $actual, float $delta = 0.0, bool $canonicalize = false, bool $ignoreCase = false): void
    {}

    private function isInfinite(mixed $value): bool
    {}

    private function isNan(mixed $value): bool
    {}
}
