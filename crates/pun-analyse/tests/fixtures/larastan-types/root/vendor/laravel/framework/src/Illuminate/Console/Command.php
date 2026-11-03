<?php

namespace Illuminate\Console;

use Illuminate\Console\Attributes\Aliases;
use Illuminate\Console\Attributes\Description;
use Illuminate\Console\Attributes\Help;
use Illuminate\Console\Attributes\Hidden;
use Illuminate\Console\Attributes\Signature;
use Illuminate\Console\Attributes\Usage;
use Illuminate\Console\View\Components\Factory;
use Illuminate\Contracts\Console\Isolatable;
use Illuminate\Support\Traits\Macroable;
use ReflectionClass;
use Symfony\Component\Console\Command\Command as SymfonyCommand;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Input\InputOption;
use Symfony\Component\Console\Output\OutputInterface;
use Throwable;

class Command extends SymfonyCommand
{
    use Concerns\CallsCommands,
        Concerns\ConfiguresPrompts,
        Concerns\HasParameters,
        Concerns\InteractsWithIO,
        Concerns\InteractsWithSignals,
        Concerns\PromptsForMissingInput,
        Macroable;

    /**
     * The Laravel application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $laravel;

    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature;

    /**
     * The console command name.
     *
     * @var string
     */
    protected $name;

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = '';

    /**
     * The console command help text.
     *
     * @var string
     */
    protected $help = '';

    /**
     * Indicates whether the command should be shown in the Artisan command list.
     *
     * @var bool
     */
    protected $hidden = false;

    /**
     * Indicates whether only one instance of the command can run at any given time.
     *
     * @var bool
     */
    protected $isolated = false;

    /**
     * The default exit code for isolated commands.
     *
     * @var self::SUCCESS|self::FAILURE|self::INVALID
     */
    protected $isolatedExitCode = self::SUCCESS;

    /**
     * The console command name aliases.
     *
     * @var string[]
     */
    protected $aliases;

    /**
     * Create a new console command instance.
     */
    public function __construct()
    {}

    /**
     * Configure argument/option defaults that can't be expressed as static signature
     * text (e.g. environment-dependent values, or non-string literal defaults such as
     * booleans or integers), by patching the already-built definition.
     */
    protected function configureDefaults(): void
    {}

    /**
     * Configure the command from class attributes.
     *
     * @return void
     */
    protected function configureFromAttributes()
    {}

    /**
     * Configure usage examples for the command from class attributes.
     *
     * @return void
     */
    protected function configureUsageFromAttribute()
    {}

    /**
     * Configure the console command using a fluent definition.
     *
     * @return void
     */
    protected function configureUsingFluentDefinition()
    {}

    /**
     * Configure the console command for isolation.
     *
     * @return void
     */
    protected function configureIsolation()
    {}

    /**
     * Run the console command.
     *
     * @param  \Symfony\Component\Console\Input\InputInterface  $input
     * @param  \Symfony\Component\Console\Output\OutputInterface  $output
     * @return int
     */
    #[\Override]
    public function run(InputInterface $input, OutputInterface $output): int
    {}

    /**
     * Execute the console command.
     *
     * @param  \Symfony\Component\Console\Input\InputInterface  $input
     * @param  \Symfony\Component\Console\Output\OutputInterface  $output
     */
    #[\Override]
    protected function execute(InputInterface $input, OutputInterface $output): int
    {}

    /**
     * Get a command isolation mutex instance for the command.
     *
     * @return \Illuminate\Console\CommandMutex
     */
    protected function commandIsolationMutex()
    {}

    /**
     * Resolve the console command instance for the given command.
     *
     * @param  \Symfony\Component\Console\Command\Command|string  $command
     * @return \Symfony\Component\Console\Command\Command
     */
    protected function resolveCommand($command)
    {}

    /**
     * Fail the command manually.
     *
     * @param  \Throwable|string|null  $exception
     * @return never
     *
     * @throws \Illuminate\Console\ManuallyFailedException|\Throwable
     */
    public function fail(Throwable|string|null $exception = null)
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    #[\Override]
    public function isHidden(): bool
    {}

    /**
     * {@inheritdoc}
     */
    #[\Override]
    public function setHidden(bool $hidden = true): static
    {}

    /**
     * Get the Laravel application instance.
     *
     * @return \Illuminate\Contracts\Foundation\Application
     */
    public function getLaravel()
    {}

    /**
     * Set the Laravel application instance.
     *
     * @param  \Illuminate\Contracts\Container\Container  $laravel
     * @return void
     */
    public function setLaravel($laravel)
    {}
}
