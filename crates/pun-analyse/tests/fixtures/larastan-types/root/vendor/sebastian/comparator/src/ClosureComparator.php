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
use function spl_object_id;
use function sprintf;
use Closure;
use ReflectionFunction;
use SebastianBergmann\Exporter\ObjectNotSupportedException;

/**
 * An object exporter is not consulted for the representation of the closures
 * that are compared: the representation this comparator provides for them
 * always has precedence.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for sebastian/comparator
 *
 * @internal This class is not covered by the backward compatibility promise for sebastian/comparator
 */
final class ClosureComparator extends Comparator
{
    public function accepts(mixed $expected, mixed $actual): bool
    {}

    /**
     * @throws ComparisonFailure
     * @throws ObjectNotSupportedException
     */
    public function assertEquals(mixed $expected, mixed $actual, float $delta = 0.0, bool $canonicalize = false, bool $ignoreCase = false): void
    {}

    private function declarationIsEqual(ReflectionFunction $expected, ReflectionFunction $actual): bool
    {}

    private function bindingIsEqual(ReflectionFunction $expected, ReflectionFunction $actual): bool
    {}

    private function capturedStateIsEqual(ReflectionFunction $expected, ReflectionFunction $actual): bool
    {}

    private function recursivelyEqual(mixed $expected, mixed $actual): bool
    {}
}
