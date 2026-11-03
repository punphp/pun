<?php

namespace Orchestra\Testbench\Foundation\Console\Actions;

use Illuminate\Console\View\Components\Factory as ComponentsFactory;
use Illuminate\Filesystem\Filesystem;
use Orchestra\Sidekick\Console\Task;

use function Laravel\Prompts\confirm;
use function Orchestra\Sidekick\Filesystem\join_paths;
use function Orchestra\Testbench\transform_realpath_to_relative;

/**
 * @api
 */
class GeneratesFile extends Action
{
    /**
     * Construct a new action instance.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $filesystem
     * @param  \Illuminate\Console\View\Components\Factory|null  $components
     * @param  bool  $force
     * @param  string|null  $workingPath
     * @param  bool  $confirmation
     * @param  bool  $pretending
     */
    public function __construct(
        public readonly Filesystem $filesystem,
        public readonly ?ComponentsFactory $components = null,
        public readonly bool $force = false,
        public ?string $workingPath = null,
        public bool $confirmation = false,
        bool $pretending = false,
    ) {}

    /**
     * Handle the action.
     *
     * @param  string|false|null  $from
     * @param  string|false|null  $to
     * @return void
     */
    public function handle(string|false|null $from, string|false|null $to): void
    {}
}
