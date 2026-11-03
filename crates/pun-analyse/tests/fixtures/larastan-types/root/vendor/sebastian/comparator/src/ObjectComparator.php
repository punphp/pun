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
use function in_array;
use function is_object;
use function sprintf;
use function substr_replace;
use SebastianBergmann\Exporter\ObjectNotSupportedException;

/**
 * When an object exporter provides the representation for the objects that
 * are compared then that representation is used for the comparison failure
 * message. This does not apply to objects for which a comparator of their own
 * exists: such a comparator is consulted before this one and the
 * representation it provides always has precedence.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for sebastian/comparator
 *
 * @internal This class is not covered by the backward compatibility promise for sebastian/comparator
 */
class ObjectComparator extends ArrayComparator
{
    public function accepts(mixed $expected, mixed $actual): bool
    {}

    /**
     * @param array<mixed> $processed
     *
     * @throws ComparisonFailure
     * @throws ObjectNotSupportedException
     */
    public function assertEquals(mixed $expected, mixed $actual, float $delta = 0.0, bool $canonicalize = false, bool $ignoreCase = false, array &$processed = []): void
    {}

    /**
     * @return array<mixed>
     */
    protected function toArray(object $object): array
    {}
}
