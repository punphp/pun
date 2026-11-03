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

use function array_is_list;
use function array_key_exists;
use function assert;
use function is_array;
use function is_bool;
use function is_float;
use function is_int;
use function is_object;
use function is_string;
use function ksort;
use function serialize;
use function sprintf;
use function str_replace;
use function trim;
use function usort;
use SebastianBergmann\Exporter\ObjectNotSupportedException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for sebastian/comparator
 *
 * @internal This class is not covered by the backward compatibility promise for sebastian/comparator
 */
class ArrayComparator extends Comparator
{
    public function accepts(mixed $expected, mixed $actual): bool
    {}

    /**
     * Arrays are equal if they contain the same key-value pairs.
     * The order of the keys does not matter.
     * The types of key-value pairs do not matter.
     *
     * @param array<mixed> $processed
     *
     * @throws ComparisonFailure
     * @throws ObjectNotSupportedException
     */
    public function assertEquals(mixed $expected, mixed $actual, float $delta = 0.0, bool $canonicalize = false, bool $ignoreCase = false, array &$processed = []): void
    {}

    private function indent(string $lines): string
    {}

    private function compare(mixed $a, mixed $b): int
    {}

    private function typeOrder(mixed $value): int
    {}
}
