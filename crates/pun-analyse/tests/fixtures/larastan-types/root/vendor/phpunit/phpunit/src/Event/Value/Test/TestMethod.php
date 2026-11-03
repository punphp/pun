<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event\Code;

use function is_int;
use function sprintf;
use PHPUnit\Event\TestData\TestDataCollection;
use PHPUnit\Metadata\MetadataCollection;
use PHPUnit\Util\Sanitizer;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestMethod extends Test
{
    /**
     * @var class-string
     */
    private string $className;

    /**
     * @var non-empty-string
     */
    private string $methodName;

    /**
     * @var non-negative-int
     */
    private int $line;
    private TestDox $testDox;
    private MetadataCollection $metadata;
    private TestDataCollection $testData;

    /**
     * @var positive-int
     */
    private int $repetition;

    /**
     * @var positive-int
     */
    private int $totalRepetitions;

    /**
     * @var positive-int
     */
    private int $attempt;

    /**
     * @var positive-int
     */
    private int $maxAttempts;

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     * @param non-empty-string $file
     * @param non-negative-int $line
     * @param positive-int     $repetition
     * @param positive-int     $totalRepetitions
     * @param positive-int     $attempt
     * @param positive-int     $maxAttempts
     *
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct(string $className, string $methodName, string $file, int $line, TestDox $testDox, MetadataCollection $metadata, TestDataCollection $testData, int $repetition = 1, int $totalRepetitions = 1, int $attempt = 1, int $maxAttempts = 1)
    {}

    /**
     * @return class-string
     */
    public function className(): string
    {}

    /**
     * @return non-empty-string
     */
    public function methodName(): string
    {}

    /**
     * @return non-negative-int
     */
    public function line(): int
    {}

    public function testDox(): TestDox
    {}

    public function metadata(): MetadataCollection
    {}

    public function testData(): TestDataCollection
    {}

    /**
     * @return positive-int
     */
    public function repetition(): int
    {}

    /**
     * @return positive-int
     */
    public function totalRepetitions(): int
    {}

    public function isRepeated(): bool
    {}

    /**
     * @return positive-int
     */
    public function attempt(): int
    {}

    /**
     * @return positive-int
     */
    public function maxAttempts(): int
    {}

    public function isRetried(): bool
    {}

    public function isTestMethod(): true
    {}

    /**
     * @return non-empty-string
     */
    public function id(): string
    {}

    /**
     * @return non-empty-string
     */
    public function nameWithClass(): string
    {}

    /**
     * @return non-empty-string
     */
    public function name(): string
    {}

    /**
     * @return non-empty-string
     */
    public function sortId(): string
    {}

    /**
     * @return non-empty-string
     */
    private function nameWithDataSet(): string
    {}
}
