<?php

namespace Orchestra\Testbench\Features;

use Closure;
use Illuminate\Support\Fluent;
use Orchestra\Testbench\Concerns\HandlesAttributes;
use Orchestra\Testbench\Pest\WithPest;
use PHPUnit\Framework\TestCase as PHPUnitTestCase;

use function Orchestra\Sidekick\once;

/**
 * @internal
 */
final class TestingFeature
{
    /**
     * Resolve available testing features for Testbench.
     *
     * @param  object  $testCase
     * @param  (\Closure():(void))|null  $default
     * @param  (\Closure():(mixed))|null  $attribute
     * @param  (\Closure(\Closure|null):(mixed))|null  $pest
     * @return \Illuminate\Support\Fluent<array-key, mixed>
     */
    public static function run(
        object $testCase,
        ?Closure $default = null,
        ?Closure $attribute = null,
        ?Closure $pest = null
    ): Fluent {}
}
