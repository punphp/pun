<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\MockObject\Generator;

use const PHP_EOL;
use const PHP_VERSION;
use function array_merge;
use function array_pop;
use function array_unique;
use function assert;
use function class_exists;
use function count;
use function explode;
use function implode;
use function in_array;
use function interface_exists;
use function is_array;
use function md5;
use function mt_rand;
use function preg_match;
use function serialize;
use function sort;
use function sprintf;
use function substr;
use function trait_exists;
use function version_compare;
use Exception;
use Iterator;
use IteratorAggregate;
use PHPUnit\Framework\MockObject\ConfigurableMethod;
use PHPUnit\Framework\MockObject\DoubledCloneMethod;
use PHPUnit\Framework\MockObject\Method;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\MockObjectApi;
use PHPUnit\Framework\MockObject\MockObjectInternal;
use PHPUnit\Framework\MockObject\ProxiedCloneMethod;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\MockObject\StubApi;
use PHPUnit\Framework\MockObject\StubInternal;
use PHPUnit\Framework\MockObject\TestDoubleState;
use PropertyHookType;
use ReflectionClass;
use ReflectionMethod;
use ReflectionObject;
use SebastianBergmann\Type\ReflectionMapper;
use SebastianBergmann\Type\Type;
use Throwable;
use Traversable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Generator
{
    use TemplateLoader;

    /**
     * @var null|non-empty-array<non-empty-string, true>
     */
    private static ?array $excludedMethodNames = null;

    /**
     * @var array<non-empty-string, DoubledClass>
     */
    private static array $cache = [];

    /**
     * Returns a test double for the specified class.
     *
     * @param class-string            $type
     * @param ?list<non-empty-string> $methods
     * @param array<mixed>            $arguments
     * @param list<non-empty-string>  $doubledProperties
     * @param ?non-empty-string       $displayName
     *
     * @throws ClassIsAnonymousException
     * @throws ClassIsEnumerationException
     * @throws ClassIsFinalException
     * @throws DuplicateMethodException
     * @throws InvalidClassNameException
     * @throws InvalidMethodNameException
     * @throws NameAlreadyInUseException
     * @throws ReflectionException
     * @throws RuntimeException
     * @throws UnknownTypeException
     */
    public function testDouble(string $type, bool $mockObject, ?array $methods = [], array $arguments = [], string $mockClassName = '', bool $callOriginalConstructor = true, bool $callOriginalClone = true, bool $returnValueGeneration = true, array $doubledProperties = [], ?string $displayName = null): MockObject|Stub
    {}

    /**
     * @param list<class-string> $interfaces
     *
     * @throws RuntimeException
     * @throws UnknownInterfaceException
     */
    public function testDoubleForInterfaceIntersection(array $interfaces, bool $mockObject, bool $returnValueGeneration = true): MockObject|Stub
    {}

    /**
     * @param class-string            $type
     * @param ?list<non-empty-string> $methods
     * @param list<non-empty-string>  $doubledProperties
     *
     * @throws ClassIsAnonymousException
     * @throws ClassIsEnumerationException
     * @throws ClassIsFinalException
     * @throws ReflectionException
     * @throws RuntimeException
     *
     * @todo This method is only public because it is used to test generated code in PHPT tests
     *
     * @see https://github.com/sebastianbergmann/phpunit/issues/5476
     */
    public function generate(string $type, bool $mockObject, ?array $methods = null, string $mockClassName = '', bool $callOriginalClone = true, array $doubledProperties = []): DoubledClass
    {}

    /**
     * @param class-string $className
     *
     * @throws ReflectionException
     *
     * @return list<DoubledMethod>
     */
    private function mockClassMethods(string $className): array
    {}

    /**
     * @param class-string $interfaceName
     *
     * @throws ReflectionException
     *
     * @return list<ReflectionMethod>
     */
    private function userDefinedInterfaceMethods(string $interfaceName): array
    {}

    /**
     * @param array<mixed>      $arguments
     * @param ?non-empty-string $displayName
     *
     * @throws ReflectionException
     * @throws RuntimeException
     */
    private function instantiate(DoubledClass $mockClass, string $type, bool $callOriginalConstructor, array $arguments, bool $returnValueGeneration, bool $isMockObject, ?string $displayName = null): object
    {}

    /**
     * @param class-string            $type
     * @param ?list<non-empty-string> $explicitMethods
     * @param list<non-empty-string>  $doubledProperties
     *
     * @throws ClassIsAnonymousException
     * @throws ClassIsEnumerationException
     * @throws ClassIsFinalException
     * @throws MethodNamedMethodException
     * @throws MethodNamedRecordInvocationsInException
     * @throws ReflectionException
     * @throws RuntimeException
     */
    private function generateCodeForTestDoubleClass(string $type, bool $mockObject, ?array $explicitMethods, string $mockClassName, bool $callOriginalClone, array $doubledProperties): DoubledClass
    {}

    /**
     * @param class-string $type
     *
     * @return array{className: class-string, originalClassName: class-string, fullClassName: class-string, namespaceName: string}
     */
    private function generateClassName(string $type, string $className, string $prefix): array
    {}

    /**
     * @param array{className: non-empty-string, originalClassName: non-empty-string, fullClassName: non-empty-string, namespaceName: string} $mockClassName
     * @param list<class-string>                                                                                                              $additionalInterfaces
     */
    private function generateTestDoubleClassDeclaration(bool $mockObject, array $mockClassName, bool $isInterface, array $additionalInterfaces, bool $isReadonly): string
    {}

    private function canMethodBeDoubled(ReflectionMethod $method): bool
    {}

    private function isMethodNameExcluded(string $name): bool
    {}

    /**
     * @throws UnknownTypeException
     */
    private function ensureKnownType(string $type): void
    {}

    /**
     * @param ?list<non-empty-string> $methods
     *
     * @throws DuplicateMethodException
     * @throws InvalidMethodNameException
     */
    private function ensureValidMethods(?array $methods): void
    {}

    /**
     * @throws InvalidClassNameException
     */
    private function ensureValidNameForTestDoubleClass(string $className): void
    {}

    /**
     * @throws NameAlreadyInUseException
     * @throws ReflectionException
     */
    private function ensureNameForTestDoubleClassIsAvailable(string $className): void
    {}

    /**
     * @template T of object
     *
     * @param class-string<T> $className
     *
     * @throws ReflectionException
     *
     * @return ReflectionClass<T>
     *
     * @phpstan-ignore throws.unusedType
     */
    private function reflectClass(string $className): ReflectionClass
    {}

    /**
     * @param class-string $classOrInterfaceName
     *
     * @throws ReflectionException
     *
     * @return list<string>
     */
    private function namesOfMethodsIn(string $classOrInterfaceName): array
    {}

    /**
     * @param list<class-string> $interfaces
     *
     * @return class-string
     */
    private function declareIntersectionInterface(array $interfaces): string
    {}

    /**
     * @param class-string $interfaceName
     *
     * @throws ReflectionException
     *
     * @return list<DoubledMethod>
     */
    private function interfaceMethods(string $interfaceName): array
    {}

    /**
     * @param list<HookedProperty> $propertiesWithHooks
     *
     * @return list<ConfigurableMethod>
     */
    private function configurableMethods(DoubledMethodSet $methods, array $propertiesWithHooks): array
    {}

    /**
     * @param ReflectionClass<object> $class
     * @param list<non-empty-string>  $doubledProperties
     *
     * @return list<HookedProperty>
     */
    private function properties(ReflectionClass $class, array $doubledProperties): array
    {}
}
