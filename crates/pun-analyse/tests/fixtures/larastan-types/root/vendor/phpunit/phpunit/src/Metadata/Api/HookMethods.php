<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Metadata\Api;

use function assert;
use function class_exists;
use function in_array;
use function sprintf;
use function strtolower;
use PHPUnit\Event\Emitter;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\After;
use PHPUnit\Metadata\AfterClass;
use PHPUnit\Metadata\Before;
use PHPUnit\Metadata\BeforeClass;
use PHPUnit\Metadata\MetadataCollection;
use PHPUnit\Metadata\Parser\Registry;
use PHPUnit\Metadata\PostCondition;
use PHPUnit\Metadata\PreCondition;
use PHPUnit\Runner\HookMethod;
use PHPUnit\Runner\HookMethodCollection;
use PHPUnit\Util\Reflection;
use ReflectionClass;
use ReflectionMethod;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @phpstan-type HookMethodsByType array{beforeClass: HookMethodCollection, before: HookMethodCollection, preCondition: HookMethodCollection, postCondition: HookMethodCollection, after: HookMethodCollection, afterClass: HookMethodCollection}
 */
final class HookMethods
{
    /**
     * @var array<class-string, HookMethodsByType>
     */
    private static array $hookMethods = [];
    private readonly Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @param class-string<TestCase> $className
     *
     * @return HookMethodsByType
     */
    public function hookMethods(string $className): array
    {}

    public function isHookMethod(ReflectionMethod $method): bool
    {}

    /**
     * @param class-string     $declaringClassName
     * @param non-empty-string $methodName
     * @param non-empty-string $attributeName
     */
    private function addHookMethod(HookMethodCollection $hookMethods, MetadataCollection $metadata, string $declaringClassName, string $methodName, string $attributeName): void
    {}

    /**
     * @return HookMethodsByType
     */
    private function emptyHookMethodsArray(): array
    {}
}
