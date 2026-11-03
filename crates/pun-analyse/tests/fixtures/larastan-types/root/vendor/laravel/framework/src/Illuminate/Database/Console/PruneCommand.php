<?php

namespace Illuminate\Database\Console;

use Illuminate\Console\Command;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Events\ModelPruningFinished;
use Illuminate\Database\Events\ModelPruningStarting;
use Illuminate\Database\Events\ModelsPruned;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use InvalidArgumentException;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Finder\Finder;

#[AsCommand(name: 'model:prune')]
class PruneCommand extends Command
{
    /**
     * The console command name.
     *
     * @var string
     */
    protected $signature = 'model:prune
                                {--model=* : Class names of the models to be pruned}
                                {--except=* : Class names of the models to be excluded from pruning}
                                {--path=* : Absolute path(s) to directories where models are located}
                                {--chunk=1000 : The number of models to retrieve per chunk of models to be deleted}
                                {--pretend : Display the number of prunable records found instead of deleting them}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Prune models that are no longer needed';

    /**
     * Execute the console command.
     *
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     * @return void
     */
    public function handle(Dispatcher $events)
    {}

    /**
     * Prune the given model.
     *
     * @param  string  $model
     * @return void
     */
    protected function pruneModel(string $model)
    {}

    /**
     * Determine the models that should be pruned.
     *
     * @return \Illuminate\Support\Collection
     *
     * @throws \InvalidArgumentException
     */
    protected function models()
    {}

    /**
     * Get the path where models are located.
     *
     * @return string[]|string
     */
    protected function getPath()
    {}

    /**
     * Display how many models will be pruned.
     *
     * @param  class-string  $model
     * @return void
     */
    protected function pretendToPrune($model)
    {}

    /**
     * Determine if the given model is prunable.
     *
     * @param  string  $model
     * @return bool
     */
    protected function isPrunable(string $model)
    {}
}
