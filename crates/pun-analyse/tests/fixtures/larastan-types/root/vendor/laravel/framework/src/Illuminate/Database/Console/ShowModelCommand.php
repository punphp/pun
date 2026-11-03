<?php

namespace Illuminate\Database\Console;

use Illuminate\Console\Concerns\FindsAvailableModels;
use Illuminate\Contracts\Console\PromptsForMissingInput;
use Illuminate\Contracts\Container\BindingResolutionException;
use Illuminate\Database\Eloquent\ModelInfo;
use Illuminate\Database\Eloquent\ModelInspector;
use Illuminate\Support\Collection;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Output\OutputInterface;

use function Laravel\Prompts\suggest;

#[AsCommand(name: 'model:show')]
class ShowModelCommand extends DatabaseInspectionCommand implements PromptsForMissingInput
{
    use FindsAvailableModels;

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Show information about an Eloquent model';

    /**
     * The console command signature.
     *
     * @var string
     */
    protected $signature = 'model:show {model : The model to show}
                {--database= : The database connection to use}
                {--json : Output the model as JSON}';

    /**
     * Execute the console command.
     *
     * @return int
     */
    public function handle(ModelInspector $modelInspector)
    {}

    /**
     * Render the model information.
     *
     * @return void
     */
    protected function display(ModelInfo $modelData)
    {}

    /**
     * Render the model information as JSON.
     *
     * @return void
     */
    protected function displayJson(ModelInfo $modelData)
    {}

    /**
     * Render the model information for the CLI.
     *
     * @return void
     */
    protected function displayCli(ModelInfo $modelData)
    {}

    /**
     * Prompt for missing input arguments using the returned questions.
     *
     * @return array<string, \Closure(): string>
     */
    protected function promptForMissingArgumentsUsing(): array
    {}
}
