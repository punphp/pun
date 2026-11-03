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

use PHPUnit\Event;
use PHPUnit\Event\Facade as EventFacade;
use PHPUnit\Runner\Phpt\TestCase as PhptTestCase;
use PHPUnit\TestRunner\TestResult\Facade as TestResultFacade;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class PhptRetryTestSuite extends PhptIterativeTestSuite
{
    /**
     * @var non-empty-string
     */
    private string $filename;

    /**
     * @var positive-int
     */
    private int $maxAttempts = 1;

    /**
     * @param non-empty-string $filename
     * @param positive-int     $maxAttempts
     */
    public static function for(string $filename, Event\Emitter $emitter, int $maxAttempts): self
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
}
