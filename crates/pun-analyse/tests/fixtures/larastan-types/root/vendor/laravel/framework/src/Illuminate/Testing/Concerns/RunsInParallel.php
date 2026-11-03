<?php

namespace Illuminate\Testing\Concerns;

use Illuminate\Contracts\Console\Kernel;
use Illuminate\Foundation\Application;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\ParallelTesting;
use Illuminate\Testing\ParallelConsoleOutput;
use PHPUnit\Event\Facade as EventFacade;
use PHPUnit\TextUI\Configuration\PhpHandler;
use RuntimeException;
use Symfony\Component\Console\Output\ConsoleOutput;
use Symfony\Component\Console\Output\OutputInterface;

trait RunsInParallel
{
    /**
     * The application resolver callback.
     *
     * @var \Closure|null
     */
    protected static $applicationResolver;

    /**
     * The runner resolver callback.
     *
     * @var \Closure|null
     */
    protected static $runnerResolver;

    /**
     * The original test runner options.
     *
     * @var \ParaTest\Runners\PHPUnit\Options|\ParaTest\Options
     */
    protected $options;

    /**
     * The output instance.
     *
     * @var \Symfony\Component\Console\Output\OutputInterface
     */
    protected $output;

    /**
     * The original test runner.
     *
     * @var \ParaTest\Runners\PHPUnit\RunnerInterface|\ParaTest\RunnerInterface
     */
    protected $runner;

    /**
     * Creates a new test runner instance.
     *
     * @param  \ParaTest\Runners\PHPUnit\Options|\ParaTest\Options  $options
     * @param  \Symfony\Component\Console\Output\OutputInterface  $output
     */
    public function __construct($options, OutputInterface $output)
    {}

    /**
     * Set the application resolver callback.
     *
     * @param  \Closure|null  $resolver
     * @return void
     */
    public static function resolveApplicationUsing($resolver)
    {}

    /**
     * Set the runner resolver callback.
     *
     * @param  \Closure|null  $resolver
     * @return void
     */
    public static function resolveRunnerUsing($resolver)
    {}

    /**
     * Runs the test suite.
     *
     * @return int
     */
    public function execute(): int
    {}

    /**
     * Returns the highest exit code encountered throughout the course of test execution.
     *
     * @return int
     */
    public function getExitCode(): int
    {}

    /**
     * Apply the given callback for each process.
     *
     * @param  callable  $callback
     * @return void
     */
    protected function forEachProcess($callback)
    {}

    /**
     * Creates the application.
     *
     * @return \Illuminate\Contracts\Foundation\Application
     *
     * @throws \RuntimeException
     */
    protected function createApplication()
    {}
}
