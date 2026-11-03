<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\ExecutionOrder\Stage;

use function array_diff;
use function array_merge;
use function array_splice;
use function count;
use PHPUnit\Framework\DataProviderTestSuite;
use PHPUnit\Framework\Test;
use PHPUnit\Framework\TestCase;
use PHPUnit\Runner\ExecutionOrder\Context;
use PHPUnit\Runner\ExecutionOrder\ReorderStage;

/**
 * Reorders the tests of a test suite in such a way as to resolve as many
 * dependencies as possible. The algorithm leaves the tests in their original
 * order when it can.
 *
 * Short description of the algorithm:
 *
 * 1. Pick the next test from the remaining tests to be checked for dependencies.
 * 2. If the test has no dependencies: mark done, start again from the top.
 * 3. If the test has dependencies but none left to do: mark done, start again from the top.
 * 4. When the end is reached, add any leftover tests to the end. These will be
 *    marked as skipped during execution.
 *
 * The tests provided by a data provider cannot depend on each other, so this
 * stage does not apply to a DataProviderTestSuite.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ResolveDependencies implements ReorderStage
{
    /**
     * @param list<Test> $tests
     *
     * @return list<Test>
     */
    public function apply(array $tests, Context $context): array
    {}

    /**
     * @return non-empty-string
     */
    public function name(): string
    {}
}
