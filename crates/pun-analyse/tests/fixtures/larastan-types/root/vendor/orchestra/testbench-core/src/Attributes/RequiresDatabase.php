<?php

namespace Orchestra\Testbench\Attributes;

use Attribute;
use Closure;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;
use InvalidArgumentException;
use Orchestra\Testbench\Contracts\Attributes\Actionable as ActionableContract;

#[Attribute(Attribute::TARGET_CLASS | Attribute::TARGET_METHOD | Attribute::IS_REPEATABLE)]
final class RequiresDatabase implements ActionableContract
{
    /**
     * Determine if currently the default connection.
     *
     * @var bool|null
     */
    public readonly ?bool $default;

    /**
     * Construct a new attribute.
     *
     * @param  string  $driver
     * @param  string|null  $versionRequirement
     * @param  string|null  $connection
     * @param  bool|null  $default
     */
    public function __construct(
        public readonly array|string $driver,
        public readonly ?string $versionRequirement = null,
        public readonly ?string $connection = null,
        ?bool $default = null
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
