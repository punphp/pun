<?php

namespace Orchestra\Testbench\Concerns;

use Closure;
use Illuminate\Support\Collection;
use Orchestra\Testbench\PHPUnit\AttributeParser;
use PHPUnit\Framework\TestCase as PHPUnitTestCase;
use PHPUnit\Metadata\Annotation\Parser\Registry as PHPUnitRegistry;
use ReflectionClass;

/**
 * @internal
 *
 * @phpstan-import-type TTestingFeature from \Orchestra\Testbench\PHPUnit\AttributeParser
 */
trait InteractsWithPHPUnit
{
    use InteractsWithTestCase;

    /**
     * The cached test case setUp resolver.
     *
     * @var (\Closure(\Closure):(void))|null
     */
    protected ?Closure $testCaseSetUpCallback = null;

    /**
     * The cached test case tearDown resolver.
     *
     * @var (\Closure(\Closure):(void))|null
     */
    protected ?Closure $testCaseTearDownCallback = null;

    /**
     * The cached class attributes for test case.
     *
     * @var array<string, array<int, array{key: class-string, instance: object}>>
     *
     * @phpstan-var array<string, array<int, array{key: class-string<TTestingFeature>, instance: TTestingFeature}>>
     */
    protected static array $cachedTestCaseClassAttributes = [];

    /**
     * The cached method attributes for test case.
     *
     * @var array<string, array<int, array{key: class-string, instance: object}>>
     *
     * @phpstan-var array<string, array<int, array{key: class-string<TTestingFeature>, instance: TTestingFeature}>>
     */
    protected static array $cachedTestCaseMethodAttributes = [];

    /**
     * Determine if the trait is used within testing.
     *
     * @api
     *
     * @return bool
     */
    public function isRunningTestCase(): bool
    {}

    /**
     * Resolve PHPUnit test class name.
     *
     * @internal
     *
     * @return class-string|null
     *
     * @codeCoverageIgnore
     */
    public function resolvePhpUnitTestClassName(): ?string
    {}

    /**
     * Resolve PHPUnit test method name.
     *
     * @internal
     *
     * @return string|null
     *
     * @codeCoverageIgnore
     */
    public function resolvePhpUnitTestMethodName(): ?string
    {}

    /**
     * Resolve PHPUnit method attributes.
     *
     * @internal
     *
     * @phpunit-overrides
     *
     * @return \Illuminate\Support\Collection<class-string, array<int, object>>
     *
     * @phpstan-return \Illuminate\Support\Collection<class-string<TTestingFeature>, array<int, TTestingFeature>>
     */
    protected function resolvePhpUnitAttributes(): Collection
    {}

    /**
     * Resolve PHPUnit method attributes for specific method.
     *
     * @internal
     *
     * @phpunit-overrides
     *
     * @param  class-string  $className
     * @param  string|null  $methodName
     * @return \Illuminate\Support\Collection<class-string, array<int, object>>
     *
     * @phpstan-return \Illuminate\Support\Collection<class-string<TTestingFeature>, array<int, TTestingFeature>>
     */
    protected static function resolvePhpUnitAttributesForMethod(string $className, ?string $methodName = null): Collection
    {}

    /**
     * Define the setUp environment using callback.
     *
     * @internal
     *
     * @param  \Closure(\Closure):void  $setUp
     * @return void
     *
     * @codeCoverageIgnore
     */
    public function setUpTheEnvironmentUsing(Closure $setUp): void
    {}

    /**
     * Define the tearDown environment using callback.
     *
     * @internal
     *
     * @param  \Closure(\Closure):void  $tearDown
     * @return void
     *
     * @codeCoverageIgnore
     */
    public function tearDownTheEnvironmentUsing(Closure $tearDown): void
    {}

    /**
     * Prepare the testing environment before the running the test case.
     *
     * @internal
     *
     * @return void
     *
     * @codeCoverageIgnore
     */
    public static function setUpBeforeClassUsingPHPUnit(): void
    {}

    /**
     * Clean up the testing environment before the next test case.
     *
     * @internal
     *
     * @return void
     *
     * @codeCoverageIgnore
     */
    public static function tearDownAfterClassUsingPHPUnit(): void
    {}
}
