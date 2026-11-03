<?php

namespace Orchestra\Testbench\Attributes;

use Attribute;
use Closure;
use Orchestra\Testbench\Contracts\Attributes\Actionable as ActionableContract;

use function Orchestra\Sidekick\laravel_version_compare;

#[Attribute(Attribute::TARGET_CLASS | Attribute::TARGET_METHOD | Attribute::IS_REPEATABLE)]
final class RequiresLaravel implements ActionableContract
{
    /**
     * Construct a new attribute.
     *
     * @param  string  $versionRequirement
     */
    public function __construct(
        public readonly string $versionRequirement
    ) {}

    /**
     * Handle the attribute.
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @param  \Closure(string, array<int, mixed>):void  $action
     * @return void
     */
    public function handle($app, Closure $action): void
    {}
}
