<?php

namespace Orchestra\Testbench\Attributes;

use Attribute;
use Illuminate\Foundation\Testing\RefreshDatabaseState;
use Orchestra\Testbench\Contracts\Attributes\AfterAll as AfterAllContract;
use Orchestra\Testbench\Contracts\Attributes\BeforeAll as BeforeAllContract;

#[Attribute(Attribute::TARGET_CLASS)]
final class ResetRefreshDatabaseState implements AfterAllContract, BeforeAllContract
{
    /**
     * Handle the attribute.
     *
     * @codeCoverageIgnore
     */
    public function beforeAll(): void
    {}

    /**
     * Handle the attribute.
     *
     * @codeCoverageIgnore
     */
    public function afterAll(): void
    {}

    /**
     * Execute the action.
     *
     * @return void
     */
    public static function run(): void
    {}
}
