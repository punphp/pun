<?php

namespace Orchestra\Testbench\Foundation\Console\Actions;

use Illuminate\Console\View\Components\Factory as ComponentsFactory;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\LazyCollection;
use Orchestra\Sidekick\Console\Task;

use function Laravel\Prompts\confirm;
use function Orchestra\Testbench\transform_realpath_to_relative;

/**
 * @api
 */
class DeleteFiles extends Action
{
    /**
     * Construct a new action instance.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $filesystem
     * @param  \Illuminate\Console\View\Components\Factory  $components
     * @param  string|null  $workingPath
     * @param  bool  $confirmation
     * @param  bool  $pretending
     */
    public function __construct(
        public readonly Filesystem $filesystem,
        public readonly ?ComponentsFactory $components = null,
        public ?string $workingPath = null,
        public bool $confirmation = false,
        bool $pretending = false,
    ) {}

    /**
     * Handle the action.
     *
     * @param  iterable<int, string>  $files
     * @return void
     */
    public function handle(iterable $files): void
    {}
}
