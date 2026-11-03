<?php

namespace Orchestra\Testbench\Attributes;

use Attribute;
use Illuminate\Support\Str;
use Orchestra\Testbench\Contracts\Attributes\Invokable as InvokableContract;

#[Attribute(Attribute::TARGET_CLASS | Attribute::TARGET_METHOD | Attribute::IS_REPEATABLE)]
final class WithConfig implements InvokableContract
{
    /**
     * List of default configuration prefix from Laravel Framework.
     *
     * @var array<int, string>
     */
    protected static array $defaultLaravelConfigurations = [
        'app.',
        'auth.',
        'broadcasting.',
        'cache.',
        'database.',
        'filesystems.',
        'logging.',
        'mail.',
        'queue.',
        'services.',
        'session.',
        'view.',
    ];

    /**
     * Construct a new attribute.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @param  bool  $defer
     */
    public function __construct(
        public readonly string $key,
        public readonly mixed $value,
        public readonly bool $defer = true
    ) {}

    /**
     * Handle the attribute.
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return void
     */
    public function __invoke($app): void
    {}

    /**
     * Determine if the given configuration key is a Laravel default configuration.
     *
     * @param  string  $configKey
     * @return bool
     */
    protected function isLaravelConfiguration(string $configKey): bool
    {}
}
