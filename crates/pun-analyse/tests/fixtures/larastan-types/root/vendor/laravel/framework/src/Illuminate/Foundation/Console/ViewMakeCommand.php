<?php

namespace Illuminate\Foundation\Console;

use Illuminate\Console\Concerns\CreatesMatchingTest;
use Illuminate\Console\GeneratorCommand;
use Illuminate\Foundation\Inspiring;
use Illuminate\Support\Facades\File;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'make:view')]
class ViewMakeCommand extends GeneratorCommand
{
    use CreatesMatchingTest;

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Create a new view';

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'make:view
                    {name : The name of the view}
                    {--extension=blade.php : The extension of the generated view}
                    {--f|force : Create the view even if the view already exists}';

    /**
     * The type of file being generated.
     *
     * @var string
     */
    protected $type = 'View';

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
     * Get the destination view path.
     *
     * @param  string  $name
     * @return string
     */
    protected function getPath($name)
    {}

    /**
     * Get the desired view name from the input.
     *
     * @return string
     */
    protected function getNameInput()
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
     * Get the destination test case path.
     *
     * @return string
     */
    protected function getTestPath()
    {}

    /**
     * Create the matching test case if requested.
     *
     * @param  string  $path
     */
    protected function handleTestCreation($path): bool
    {}

    /**
     * Get the namespace for the test.
     *
     * @return string
     */
    protected function testNamespace()
    {}

    /**
     * Get the class name for the test.
     *
     * @return string
     */
    protected function testClassName()
    {}

    /**
     * Get the class fully-qualified name for the test.
     *
     * @return string
     */
    protected function testClassFullyQualifiedName()
    {}

    /**
     * Get the test stub file for the generator.
     *
     * @return string
     */
    protected function getTestStub()
    {}

    /**
     * Get the view name for the test.
     *
     * @return string
     */
    protected function testViewName()
    {}

    /**
     * Determine if Pest is being used by the application.
     *
     * @return bool
     */
    protected function usingPest()
    {}
}
