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

use PHPUnit\Event\TestData\DataFromDataProvider;
use PHPUnit\Event\TestData\DataFromTestDependency;
use PHPUnit\Event\TestData\TestDataCollection;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\MetadataCollection;
use PHPUnit\Metadata\Parser\Registry as MetadataRegistry;
use PHPUnit\Util\Exporter;
use PHPUnit\Util\Reflection;
use PHPUnit\Util\Test as TestUtil;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestMethodBuilder
{
    public static function fromTestCase(TestCase $testCase, bool $useTestCaseForTestDox = true): TestMethod
    {}

    /**
     * @throws NoTestCaseObjectOnCallStackException
     */
    public static function fromCallStack(): TestMethod
    {}

    /**
     * The metadata of the test method, without the closure that a
     * #[DataProviderClosure] attribute declares. The closure is only used to
     * build the test suite, and an event that carries it cannot be serialized,
     * while the events of a test that runs in a child process are serialized
     * to be forwarded to the main process.
     *
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    private static function metadataFor(string $className, string $methodName): MetadataCollection
    {}

    private static function dataFor(TestCase $testCase): TestDataCollection
    {}
}
