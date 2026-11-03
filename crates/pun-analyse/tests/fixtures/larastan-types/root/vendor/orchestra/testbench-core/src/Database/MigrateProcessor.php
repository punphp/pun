<?php

namespace Orchestra\Testbench\Database;

use Orchestra\Testbench\Contracts\TestCase;

use function Orchestra\Testbench\artisan;

/**
 * @internal
 */
class MigrateProcessor
{
    /**
     * Construct a new schema migrator.
     *
     * @param  \Orchestra\Testbench\Contracts\TestCase  $testbench
     * @param  array<string, mixed>  $options
     */
    public function __construct(
        protected readonly TestCase $testbench,
        protected readonly array $options = []
    ) {}

    /**
     * Run migration.
     *
     * @return $this
     */
    public function up()
    {}

    /**
     * Rollback migration.
     *
     * @return $this
     */
    public function rollback()
    {}

    /**
     * Dispatch artisan command.
     *
     * @param  string  $command
     * @return void
     */
    protected function dispatch(string $command): void
    {}
}
