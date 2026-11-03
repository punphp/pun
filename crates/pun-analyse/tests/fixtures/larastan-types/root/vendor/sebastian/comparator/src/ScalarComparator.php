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

use function assert;
use function is_bool;
use function is_object;
use function is_scalar;
use function is_string;
use function mb_strtolower;
use function method_exists;
use function sprintf;
use function strlen;
use function substr;
use SebastianBergmann\Exporter\ObjectNotSupportedException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for sebastian/comparator
 *
 * @internal This class is not covered by the backward compatibility promise for sebastian/comparator
 */
class ScalarComparator extends Comparator
{
    private const int OVERLONG_THRESHOLD = 40;
    private const int KEEP_CONTEXT_CHARS = 25;

    public function accepts(mixed $expected, mixed $actual): bool
    {}

    /**
     * @throws ComparisonFailure
     * @throws ObjectNotSupportedException
     */
    public function assertEquals(mixed $expected, mixed $actual, float $delta = 0.0, bool $canonicalize = false, bool $ignoreCase = false): void
    {}

    /**
     * @return array{string, string}
     */
    private static function removeOverlongCommonPrefix(string $string1, string $string2): array
    {}

    private static function findCommonPrefix(string $string1, string $string2): string
    {}

    /**
     * @return array{string, string}
     */
    private static function removeOverlongCommonSuffix(string $string1, string $string2): array
    {}

    private static function findCommonSuffix(string $string1, string $string2): string
    {}
}
