<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\TestCase;

use function class_exists;
use function is_object;
use function sprintf;
use DeepCopy\DeepCopy;
use PHPUnit\Event;
use PHPUnit\Framework\ExecutionOrderDependency;
use PHPUnit\Framework\InvalidDependencyException;
use PHPUnit\Framework\TestCase;
use PHPUnit\Framework\TestStatus\TestStatus;
use PHPUnit\TestRunner\TestResult\PassedTests;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class DependencyResolver
{
    /**
     * Returns true when the test can be run.
     *
     * Returns false when the test cannot be run because one of its
     * dependencies does not exist or did not pass. The test's status is set
     * and the corresponding event is emitted in that case.
     *
     * @param list<ExecutionOrderDependency> $dependencies
     */
    public function resolve(TestCase $test, array $dependencies, Event\Emitter $emitter): bool
    {}

    private function markErrorForInvalidDependency(TestCase $test, ExecutionOrderDependency $dependency, Event\Emitter $emitter): void
    {}

    private function markSkippedForMissingDependency(TestCase $test, ExecutionOrderDependency $dependency, Event\Emitter $emitter): void
    {}
}
