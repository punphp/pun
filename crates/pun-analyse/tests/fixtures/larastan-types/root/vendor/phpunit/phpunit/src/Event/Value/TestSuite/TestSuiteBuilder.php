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

use function assert;
use function class_exists;
use function count;
use function explode;
use function method_exists;
use function strpos;
use function substr;
use PHPUnit\Event\Code\Test;
use PHPUnit\Event\Code\TestCollection;
use PHPUnit\Event\Code\TestDoxBuilder;
use PHPUnit\Event\RuntimeException;
use PHPUnit\Framework\DataProviderTestSuite;
use PHPUnit\Framework\IterativeTestSuite;
use PHPUnit\Framework\PhptRepeatTestSuite;
use PHPUnit\Framework\PhptRetryTestSuite;
use PHPUnit\Framework\RetryTestSuite;
use PHPUnit\Framework\TestCase;
use PHPUnit\Framework\TestSuite as FrameworkTestSuite;
use PHPUnit\Runner\Phpt\TestCase as PhptTestCase;
use ReflectionClass;
use ReflectionMethod;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestSuiteBuilder
{
    /**
     * @throws RuntimeException
     */
    public static function from(FrameworkTestSuite $testSuite): TestSuite
    {}

    /**
     * @param list<Test> $tests
     */
    private static function process(FrameworkTestSuite $testSuite, array &$tests): void
    {}
}
