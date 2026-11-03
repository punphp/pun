<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework;

use function assert;
use Closure;
use PHPUnit\Event;
use PHPUnit\Event\NoPreviousThrowableException;
use PHPUnit\TestRunner\TestResult\Facade as TestResultFacade;
use SebastianBergmann\CodeCoverage\InvalidArgumentException;
use SebastianBergmann\CodeCoverage\UnintentionallyCoveredCodeException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class RetryTestSuite extends IterativeTestSuite
{
    /**
     * @var positive-int
     */
    private int $maxAttempts = 1;

    /**
     * @var ?Closure(): TestCase
     */
    private ?Closure $additionalAttemptFactory = null;

    /**
     * @param non-empty-string       $name
     * @param positive-int           $maxAttempts
     * @param Closure(): TestCase    $additionalAttemptFactory
     * @param list<non-empty-string> $groups
     */
    public static function fromTestCase(string $name, Event\Emitter $emitter, TestCase $test, int $maxAttempts, Closure $additionalAttemptFactory, array $groups = []): self
    {}

    /**
     * @return positive-int
     */
    public function maxAttempts(): int
    {}

    /**
     * @param list<Test> $tests
     */
    protected function execute(array $tests, Event\Emitter $emitter): void
    {}

    /**
     * @throws Event\RuntimeException
     * @throws Exception
     * @throws InvalidArgumentException
     * @throws NoPreviousThrowableException
     * @throws UnintentionallyCoveredCodeException
     */
    private function runAttempts(TestCase $test, Event\Emitter $emitter): void
    {}
}
