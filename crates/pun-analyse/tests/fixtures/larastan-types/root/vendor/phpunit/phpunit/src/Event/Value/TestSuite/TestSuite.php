<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event\TestSuite;

use PHPUnit\Event\Code\TestCollection;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
abstract readonly class TestSuite
{
    /**
     * @var non-empty-string
     */
    private string $name;
    private int $count;
    private TestCollection $tests;

    /**
     * @param non-empty-string $name
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(string $name, int $size, TestCollection $tests)
    {}

    /**
     * @return non-empty-string
     */
    public function name(): string
    {}

    public function count(): int
    {}

    public function tests(): TestCollection
    {}

    /**
     * @phpstan-assert-if-true TestSuiteWithName $this
     */
    public function isWithName(): bool
    {}

    /**
     * @phpstan-assert-if-true TestSuiteForTestClass $this
     */
    public function isForTestClass(): bool
    {}

    /**
     * @phpstan-assert-if-true TestSuiteForTestMethodWithDataProvider $this
     */
    public function isForTestMethodWithDataProvider(): bool
    {}

    /**
     * @phpstan-assert-if-true TestSuiteForRepeatedTestMethod $this
     */
    public function isForRepeatedTestMethod(): bool
    {}

    /**
     * @phpstan-assert-if-true TestSuiteForRetriedTestMethod $this
     */
    public function isForRetriedTestMethod(): bool
    {}

    /**
     * @phpstan-assert-if-true TestSuiteForRepeatedPhpt $this
     */
    public function isForRepeatedPhpt(): bool
    {}

    /**
     * @phpstan-assert-if-true TestSuiteForRetriedPhpt $this
     */
    public function isForRetriedPhpt(): bool
    {}
}
