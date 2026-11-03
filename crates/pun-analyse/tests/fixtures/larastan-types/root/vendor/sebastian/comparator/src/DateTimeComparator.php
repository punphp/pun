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
use function floor;
use function in_array;
use function sprintf;
use DateInterval;
use DateTime;
use DateTimeImmutable;
use DateTimeZone;

/**
 * An object exporter is not consulted for the representation of the
 * DateTime and DateTimeImmutable objects that are compared: the
 * representation this comparator provides for them always has precedence.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for sebastian/comparator
 *
 * @internal This class is not covered by the backward compatibility promise for sebastian/comparator
 */
final class DateTimeComparator extends ObjectComparator
{
    public function accepts(mixed $expected, mixed $actual): bool
    {}

    /**
     * @param array<mixed> $processed
     *
     * @throws ComparisonFailure
     */
    public function assertEquals(mixed $expected, mixed $actual, float $delta = 0.0, bool $canonicalize = false, bool $ignoreCase = false, array &$processed = []): void
    {}
}
