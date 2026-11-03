<?php

namespace Illuminate\Testing;

use Illuminate\Console\OutputStyle;
use Illuminate\Console\PromptValidationException;
use Illuminate\Contracts\Console\Kernel;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\Tappable;
use Laravel\Prompts\Note as PromptsNote;
use Laravel\Prompts\Prompt as BasePrompt;
use Laravel\Prompts\Table as PromptsTable;
use Mockery;
use Mockery\Exception\NoMatchingExpectationException;
use PHPUnit\Framework\TestCase as PHPUnitTestCase;
use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Helper\Table;
use Symfony\Component\Console\Input\ArrayInput;
use Symfony\Component\Console\Output\BufferedOutput;
use Symfony\Component\Console\Output\ConsoleOutput;
use Symfony\Component\Console\Question\ChoiceQuestion;

class PendingCommand
{
    use Conditionable, Macroable, Tappable;

    /**
     * The test being run.
     *
     * @var \Illuminate\Foundation\Testing\TestCase
     */
    public $test;

    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $app;

    /**
     * The command to run.
     *
     * @var string
     */
    protected $command;

    /**
     * The parameters to pass to the command.
     *
     * @var array
     */
    protected $parameters;

    /**
     * The expected exit code.
     *
     * @var int
     */
    protected $expectedExitCode;

    /**
     * The unexpected exit code.
     *
     * @var int
     */
    protected $unexpectedExitCode;

    /**
     * Determine if the command has executed.
     *
     * @var bool
     */
    protected $hasExecuted = false;

    /**
     * Create a new pending console command run.
     *
     * @param  \PHPUnit\Framework\TestCase  $test
     * @param  \Illuminate\Contracts\Container\Container  $app
     * @param  string  $command
     * @param  array  $parameters
     */
    public function __construct(PHPUnitTestCase $test, Container $app, $command, $parameters)
    {}

    /**
     * Specify an expected question that will be asked when the command runs.
     *
     * @param  string  $question
     * @param  array|string|bool  $answer
     * @return $this
     */
    public function expectsQuestion($question, $answer)
    {}

    /**
     * Specify an expected confirmation question that will be asked when the command runs.
     *
     * @param  string  $question
     * @param  string  $answer
     * @return $this
     */
    public function expectsConfirmation($question, $answer = 'no')
    {}

    /**
     * Specify an expected choice question with expected answers that will be asked/shown when the command runs.
     *
     * @param  string  $question
     * @param  string|array  $answer
     * @param  array  $answers
     * @param  bool  $strict
     * @return $this
     */
    public function expectsChoice($question, $answer, $answers, $strict = false)
    {}

    /**
     * Specify an expected search question with an expected search string, followed by an expected choice question with expected answers.
     *
     * @param  string  $question
     * @param  string|array  $answer
     * @param  string  $search
     * @param  array  $answers
     * @return $this
     */
    public function expectsSearch($question, $answer, $search, $answers)
    {}

    /**
     * Specify output that should be printed when the command runs.
     *
     * @param  string|null  $output
     * @return $this
     */
    public function expectsOutput($output = null)
    {}

    /**
     * Specify output that should never be printed when the command runs.
     *
     * @param  string|null  $output
     * @return $this
     */
    public function doesntExpectOutput($output = null)
    {}

    /**
     * Specify that the given string should be contained in the command output.
     *
     * @param  string  $string
     * @return $this
     */
    public function expectsOutputToContain($string)
    {}

    /**
     * Specify that the given string shouldn't be contained in the command output.
     *
     * @param  string  $string
     * @return $this
     */
    public function doesntExpectOutputToContain($string)
    {}

    /**
     * Specify a table that should be printed when the command runs.
     *
     * @param  array  $headers
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $rows
     * @param  string  $tableStyle
     * @param  array  $columnStyles
     * @return $this
     */
    public function expectsTable($headers, $rows, $tableStyle = 'default', array $columnStyles = [])
    {}

    /**
     * Specify that the given Prompts info message should be contained in the command output.
     *
     * @return $this
     */
    public function expectsPromptsInfo(string $message)
    {}

    /**
     * Specify that the given Prompts warning message should be contained in the command output.
     *
     * @return $this
     */
    public function expectsPromptsWarning(string $message)
    {}

    /**
     * Specify that the given Prompts error message should be contained in the command output.
     *
     * @return $this
     */
    public function expectsPromptsError(string $message)
    {}

    /**
     * Specify that the given Prompts alert message should be contained in the command output.
     *
     * @return $this
     */
    public function expectsPromptsAlert(string $message)
    {}

    /**
     * Specify that the given Prompts intro message should be contained in the command output.
     *
     * @return $this
     */
    public function expectsPromptsIntro(string $message)
    {}

    /**
     * Specify that the given Prompts outro message should be contained in the command output.
     *
     * @return $this
     */
    public function expectsPromptsOutro(string $message)
    {}

    /**
     * Specify a Prompts table that should be printed when the command runs.
     *
     * @param  array<int, string|array<int, string>>|Collection<int, string|array<int, string>>  $headers
     * @param  array<int, array<int, string>>|Collection<int, array<int, string>>|null  $rows
     *
     * @phpstan-param ($rows is null ? list<list<string>>|Collection<int, list<string>> : list<string|list<string>>|Collection<int, string|list<string>>) $headers
     *
     * @return $this
     */
    public function expectsPromptsTable(array|Collection $headers, array|Collection|null $rows)
    {}

    /**
     * Render the given prompt and add the output to the expectations.
     *
     * @return void
     */
    protected function expectOutputToContainPrompt(BasePrompt $prompt)
    {}

    /**
     * Assert that the command has the given exit code.
     *
     * @param  int  $exitCode
     * @return $this
     */
    public function assertExitCode($exitCode)
    {}

    /**
     * Assert that the command does not have the given exit code.
     *
     * @param  int  $exitCode
     * @return $this
     */
    public function assertNotExitCode($exitCode)
    {}

    /**
     * Assert that the command has the success exit code.
     *
     * @return $this
     */
    public function assertSuccessful()
    {}

    /**
     * Assert that the command has the success exit code.
     *
     * @return $this
     */
    public function assertOk()
    {}

    /**
     * Assert that the command does not have the success exit code.
     *
     * @return $this
     */
    public function assertFailed()
    {}

    /**
     * Execute the command.
     *
     * @return int
     */
    public function execute()
    {}

    /**
     * Execute the command.
     *
     * @return int
     *
     * @throws \Mockery\Exception\NoMatchingExpectationException
     */
    public function run()
    {}

    /**
     * Debug the command.
     *
     * @return never
     */
    public function dd()
    {}

    /**
     * Determine if expected questions / choices / outputs are fulfilled.
     *
     * @return void
     */
    protected function verifyExpectations()
    {}

    /**
     * Mock the application's console output.
     *
     * @return \Mockery\MockInterface
     */
    protected function mockConsoleOutput()
    {}

    /**
     * Create a mock for the buffered output.
     *
     * @return \Mockery\MockInterface
     */
    private function createABufferedOutputMock()
    {}

    /**
     * Flush the expectations from the test case.
     *
     * @return void
     */
    protected function flushExpectations()
    {}

    /**
     * Handle the object's destruction.
     *
     * @return void
     */
    public function __destruct()
    {}
}
