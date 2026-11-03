<?php

namespace Orchestra\Testbench\Foundation\Process;

use Closure;
use Illuminate\Support\Arr;
use Illuminate\Support\ProcessUtils;
use Laravel\SerializableClosure\SerializableClosure;
use Symfony\Component\Process\Process;

use function Orchestra\Testbench\defined_environment_variables;
use function Orchestra\Testbench\php_binary;

/**
 * @internal
 */
final class RemoteCommand
{
    /**
     * Construct a new action.
     *
     * @param  string  $workingPath
     * @param  array<string, mixed>|string  $env
     * @param  bool|null  $tty
     */
    public function __construct(
        public string $workingPath,
        public array|string $env = [],
        public ?bool $tty = null,
    ) {}

    /**
     * Execute the command.
     *
     * @param  (\Closure():(mixed))|array<int, string>|string  $command
     * @return \Orchestra\Testbench\Foundation\Process\ProcessDecorator
     */
    public function handle(string $commander, Closure|array|string $command): ProcessDecorator
    {}
}
