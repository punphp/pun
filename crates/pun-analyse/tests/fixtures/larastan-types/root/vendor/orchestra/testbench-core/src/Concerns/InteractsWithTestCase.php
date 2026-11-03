<?php

namespace Orchestra\Testbench\Concerns;

use Attribute;
use Illuminate\Foundation\Testing\LazilyRefreshDatabase;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Support\Collection;
use Orchestra\Testbench\Contracts\Attributes\AfterAll as AfterAllContract;
use Orchestra\Testbench\Contracts\Attributes\AfterEach as AfterEachContract;
use Orchestra\Testbench\Contracts\Attributes\BeforeAll as BeforeAllContract;
use Orchestra\Testbench\Contracts\Attributes\BeforeEach as BeforeEachContract;
use Orchestra\Testbench\Contracts\Attributes\Resolvable as ResolvableContract;
use Orchestra\Testbench\PHPUnit\AttributeParser;

use function Orchestra\Testbench\laravel_or_fail;

/**
 * @phpstan-import-type TTestingFeature from \Orchestra\Testbench\PHPUnit\AttributeParser
 * @phpstan-import-type TAttributes from \Orchestra\Testbench\PHPUnit\AttributeParser
 */
trait InteractsWithTestCase
{
    /**
     * The cached application bootstrap file.
     *
     * @var string|null
     */
    protected static string|bool|null $cacheApplicationBootstrapFile = null;

    /**
     * The cached uses for test case.
     *
     * @var array<class-string, class-string>|null
     */
    protected static ?array $cachedTestCaseUses = null;

    /**
     * The method attributes for test case.
     *
     * @var array<int, array{key: class-string, instance: object}>
     *
     * @phpstan-var array<int, array{key: class-string<TTestingFeature>, instance: TTestingFeature}>
     */
    protected static array $testCaseTestingFeatures = [];

    /**
     * The method attributes for test case's method.
     *
     * @var array<int, array{key: class-string, instance: object}>
     *
     * @phpstan-var array<int, array{key: class-string<TTestingFeature>, instance: TTestingFeature}>
     */
    protected static array $testCaseMethodTestingFeatures = [];

    /**
     * Determine if the trait is using given trait (or default to \Orchestra\Testbench\Concerns\Testing trait).
     *
     * @api
     *
     * @param  class-string|null  $trait
     * @return bool
     */
    public static function usesTestingConcern(?string $trait = null): bool
    {}

    /**
     * Determine if the trait is using \Illuminate\Foundation\Testing\LazilyRefreshDatabase or \Illuminate\Foundation\Testing\RefreshDatabase trait.
     *
     * @return bool
     */
    public static function usesRefreshDatabaseTestingConcern(): bool
    {}

    /**
     * Define or get the cached uses for test case.
     *
     * @internal
     *
     * @return array<class-string, class-string>
     */
    public static function cachedUsesForTestCase(): array
    {}

    /**
     * Uses testing feature (attribute) on the current test.
     *
     * @api
     *
     * @param  object  $attribute
     * @param  int  $flag
     *
     * @phpstan-param TAttributes $attribute
     *
     * @return void
     */
    public static function usesTestingFeature($attribute, int $flag = Attribute::TARGET_CLASS): void
    {}

    /**
     * Resolve PHPUnit method attributes for specific method.
     *
     * @phpunit-overrides
     *
     * @param  class-string  $className
     * @param  string|null  $methodName
     * @return \Illuminate\Support\Collection<class-string, array<int, object>>
     *
     * @phpstan-return \Illuminate\Support\Collection<class-string<TTestingFeature>, array<int, TTestingFeature>>
     */
    abstract protected static function resolvePhpUnitAttributesForMethod(string $className, ?string $methodName = null): Collection;

    /**
     * Prepare the testing environment before the running the test case.
     *
     * @internal
     *
     * @return void
     */
    protected function setUpTheTestEnvironmentUsingTestCase(): void
    {}

    /**
     * Prepare the testing environment before the running the test case.
     *
     * @internal
     *
     * @return void
     */
    protected function tearDownTheTestEnvironmentUsingTestCase(): void
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
    public static function setUpBeforeClassUsingTestCase(): void
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
    public static function tearDownAfterClassUsingTestCase(): void
    {}
}
