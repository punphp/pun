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
use function range;
use PHPUnit\Event;
use PHPUnit\Event\Facade as EventFacade;
use PHPUnit\Runner\Phpt\TestCase as PhptTestCase;
use PHPUnit\TestRunner\TestResult\Facade as TestResultFacade;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class PhptRepeatTestSuite extends PhptIterativeTestSuite
{
    /**
     * @param non-empty-string       $filename
     * @param positive-int           $numberOfRuns
     * @param list<non-empty-string> $groups
     */
    public static function for(string $filename, Event\Emitter $emitter, int $numberOfRuns, array $groups = []): self
    {}

    /**
     * @param list<Test> $tests
     */
    protected function execute(array $tests, Event\Emitter $emitter): void
    {}
}
