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

use PHPUnit\Event\Facade;
use PHPUnit\Framework\TestCase;
use PHPUnit\Logging\TestDox\NamePrettifier;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class TestDoxBuilder
{
    private static ?NamePrettifier $namePrettifier = null;

    public static function fromTestCase(TestCase $testCase): TestDox
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public static function fromClassNameAndMethodName(string $className, string $methodName): TestDox
    {}

    /**
     * @param class-string $className
     */
    public static function prettifyClassName(string $className): string
    {}

    private static function namePrettifier(): NamePrettifier
    {}
}
