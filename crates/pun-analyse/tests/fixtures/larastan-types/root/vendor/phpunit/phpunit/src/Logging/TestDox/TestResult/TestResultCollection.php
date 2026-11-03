<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Logging\TestDox;

use IteratorAggregate;

/**
 * @template-implements IteratorAggregate<non-negative-int, TestResult>
 *
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestResultCollection implements IteratorAggregate
{
    /**
     * @var list<TestResult>
     */
    private array $testResults;

    /**
     * @param list<TestResult> $testResults
     */
    public static function fromArray(array $testResults): self
    {}

    private function __construct(TestResult ...$testResults)
    {}

    /**
     * @return list<TestResult>
     */
    public function asArray(): array
    {}

    public function getIterator(): TestResultCollectionIterator
    {}
}
