<?php

namespace Illuminate\Foundation\Console;

use Illuminate\Console\Concerns\CreatesMatchingTest;
use Illuminate\Console\GeneratorCommand;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Output\OutputInterface;

use function Laravel\Prompts\confirm;
use function Laravel\Prompts\multiselect;

#[AsCommand(name: 'make:model')]
class ModelMakeCommand extends GeneratorCommand
{
    use CreatesMatchingTest;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'make:model
                    {name : The name of the model}
                    {--a|all : Generate a migration, seeder, factory, policy, resource controller, and form request classes for the model}
                    {--c|controller : Create a new controller for the model}
                    {--f|factory : Create a new factory for the model}
                    {--force : Create the class even if the model already exists}
                    {--m|migration : Create a new migration file for the model}
                    {--morph-pivot : Indicates if the generated model should be a custom polymorphic intermediate table model}
                    {--policy : Create a new policy for the model}
                    {--s|seed : Create a new seeder for the model}
                    {--p|pivot : Indicates if the generated model should be a custom intermediate table model}
                    {--r|resource : Indicates if the generated controller should be a resource controller}
                    {--api : Indicates if the generated controller should be an API resource controller}
                    {--R|requests : Create new form request classes and use them in the resource controller}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Create a new Eloquent model class';

    /**
     * The type of class being generated.
     *
     * @var string
     */
    protected $type = 'Model';

    /**
     * Execute the console command.
     *
     * @return bool|null
     */
    public function handle()
    {}

    /**
     * Create a model factory for the model.
     *
     * @return void
     */
    protected function createFactory()
    {}

    /**
     * Create a migration file for the model.
     *
     * @return void
     */
    protected function createMigration()
    {}

    /**
     * Create a seeder file for the model.
     *
     * @return void
     */
    protected function createSeeder()
    {}

    /**
     * Create a controller for the model.
     *
     * @return void
     */
    protected function createController()
    {}

    /**
     * Create the form requests for the model.
     *
     * @return void
     */
    protected function createFormRequests()
    {}

    /**
     * Create a policy file for the model.
     *
     * @return void
     */
    protected function createPolicy()
    {}

    /**
     * Get the stub file for the generator.
     *
     * @return string
     */
    protected function getStub()
    {}

    /**
     * Resolve the fully-qualified path to the stub.
     *
     * @param  string  $stub
     * @return string
     */
    protected function resolveStubPath($stub)
    {}

    /**
     * Get the default namespace for the class.
     *
     * @param  string  $rootNamespace
     * @return string
     */
    protected function getDefaultNamespace($rootNamespace)
    {}

    /**
     * Build the class with the given name.
     *
     * @param  string  $name
     * @return string
     *
     * @throws \Illuminate\Contracts\Filesystem\FileNotFoundException
     */
    protected function buildClass($name)
    {}

    /**
     * Build the replacements for a factory.
     *
     * @return array<string, string>
     */
    protected function buildFactoryReplacements()
    {}

    /**
     * Interact further with the user if they were prompted for missing arguments.
     *
     * @param  \Symfony\Component\Console\Input\InputInterface  $input
     * @param  \Symfony\Component\Console\Output\OutputInterface  $output
     * @return void
     */
    protected function afterPromptingForMissingArguments(InputInterface $input, OutputInterface $output)
    {}
}
