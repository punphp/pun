<?php

namespace Illuminate\Console\Concerns;

use Closure;
use Illuminate\Console\CommandInput;
use Illuminate\Console\OutputStyle;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Support\Str;
use Symfony\Component\Console\Formatter\OutputFormatterStyle;
use Symfony\Component\Console\Helper\Table;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Console\Question\ChoiceQuestion;
use Symfony\Component\Console\Question\Question;

trait InteractsWithIO
{
    /**
     * The console components factory.
     *
     * @var \Illuminate\Console\View\Components\Factory
     */
    protected $components;

    /**
     * The input interface implementation.
     *
     * @var \Symfony\Component\Console\Input\InputInterface
     */
    protected $input;

    /**
     * The output interface implementation.
     *
     * @var \Illuminate\Console\OutputStyle
     */
    protected $output;

    /**
     * The default verbosity of output commands.
     *
     * @var \Symfony\Component\Console\Output\OutputInterface::VERBOSITY_*
     */
    protected $verbosity = OutputInterface::VERBOSITY_NORMAL;

    /**
     * The mapping between human-readable verbosity levels and Symfony's OutputInterface.
     *
     * @var array<string, \Symfony\Component\Console\Output\OutputInterface::VERBOSITY_*>
     */
    protected $verbosityMap = [
        'v' => OutputInterface::VERBOSITY_VERBOSE,
        'vv' => OutputInterface::VERBOSITY_VERY_VERBOSE,
        'vvv' => OutputInterface::VERBOSITY_DEBUG,
        'quiet' => OutputInterface::VERBOSITY_QUIET,
        'normal' => OutputInterface::VERBOSITY_NORMAL,
    ];

    /**
     * Retrieve the command's input as a CommandInput instance or retrieve an input item.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return ($key is null ? \Illuminate\Console\CommandInput : mixed)
     */
    public function input($key = null, $default = null)
    {}

    /**
     * Determine if the given argument is present.
     *
     * @param  string|int  $name
     * @return bool
     */
    public function hasArgument($name)
    {}

    /**
     * Get the value of a command argument.
     *
     * @param  string|null  $key
     * @return ($key is null ? array<array|string|float|int|bool|null> : array|string|float|int|bool|null)
     */
    public function argument($key = null)
    {}

    /**
     * Get all of the arguments passed to the command.
     *
     * @return array<array|string|float|int|bool|null>
     */
    public function arguments()
    {}

    /**
     * Determine whether the option is defined in the command signature.
     *
     * @param  string  $name
     * @return bool
     */
    public function hasOption($name)
    {}

    /**
     * Get the value of a command option.
     *
     * @param  string|null  $key
     * @return ($key is null ? array<array|string|float|int|bool|null> : array|string|float|int|bool|null)
     */
    public function option($key = null)
    {}

    /**
     * Get all of the options passed to the command.
     *
     * @return array<array|string|float|int|bool|null>
     */
    public function options()
    {}

    /**
     * Confirm a question with the user.
     *
     * @param  string  $question
     * @param  bool  $default
     * @return bool
     */
    public function confirm($question, $default = false)
    {}

    /**
     * Prompt the user for input.
     *
     * @param  string  $question
     * @param  string|null  $default
     * @return mixed
     */
    public function ask($question, $default = null)
    {}

    /**
     * Prompt the user for input with auto completion.
     *
     * @param  string  $question
     * @param  array|callable  $choices
     * @param  string|null  $default
     * @return mixed
     */
    public function anticipate($question, $choices, $default = null)
    {}

    /**
     * Prompt the user for input with auto completion.
     *
     * @param  string  $question
     * @param  iterable|(callable(string): list<string>)  $choices
     * @param  string|null  $default
     * @return mixed
     */
    public function askWithCompletion($question, $choices, $default = null)
    {}

    /**
     * Prompt the user for input but hide the answer from the console.
     *
     * @param  string  $question
     * @param  bool  $fallback
     * @return mixed
     */
    public function secret($question, $fallback = true)
    {}

    /**
     * Give the user a single choice from an array of answers.
     *
     * @param  string  $question
     * @param  array<\Stringable|string|float|int|bool>  $choices
     * @param  string|int|null  $default
     * @param  ?positive-int  $attempts
     * @param  bool  $multiple
     * @return string|array
     */
    public function choice($question, array $choices, $default = null, $attempts = null, $multiple = false)
    {}

    /**
     * Format input to textual table.
     *
     * @param  array  $headers
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $rows
     * @param  \Symfony\Component\Console\Helper\TableStyle|string  $tableStyle
     * @param  array<int, \Symfony\Component\Console\Helper\TableStyle|string>  $columnStyles
     * @return void
     */
    public function table($headers, $rows, $tableStyle = 'default', array $columnStyles = [])
    {}

    /**
     * Execute a given callback while advancing a progress bar.
     *
     * @template TKey of array-key
     * @template TValue
     * @template TIterable of iterable<TKey, TValue>
     *
     * @param  TIterable|int  $totalSteps
     * @param  \Closure(\Symfony\Component\Console\Helper\ProgressBar): mixed|\Closure(TValue, \Symfony\Component\Console\Helper\ProgressBar, TKey): mixed  $callback
     * @return ($totalSteps is iterable ? TIterable : void)
     */
    public function withProgressBar($totalSteps, Closure $callback)
    {}

    /**
     * Write a string as information output.
     *
     * @param  string  $string
     * @param  'v'|'vv'|'vvv'|'quiet'|'normal'|\Symfony\Component\Console\Output\OutputInterface::VERBOSITY_*|null  $verbosity
     * @return void
     */
    public function info($string, $verbosity = null)
    {}

    /**
     * Write a string as standard output.
     *
     * @param  string  $string
     * @param  'info'|'comment'|'question'|'error'|'warn'|'alert'|null  $style
     * @param  'v'|'vv'|'vvv'|'quiet'|'normal'|\Symfony\Component\Console\Output\OutputInterface::VERBOSITY_*|null  $verbosity
     * @return void
     */
    public function line($string, $style = null, $verbosity = null)
    {}

    /**
     * Write a string as comment output.
     *
     * @param  string  $string
     * @param  'v'|'vv'|'vvv'|'quiet'|'normal'|\Symfony\Component\Console\Output\OutputInterface::VERBOSITY_*|null  $verbosity
     * @return void
     */
    public function comment($string, $verbosity = null)
    {}

    /**
     * Write a string as question output.
     *
     * @param  string  $string
     * @param  'v'|'vv'|'vvv'|'quiet'|'normal'|\Symfony\Component\Console\Output\OutputInterface::VERBOSITY_*|null  $verbosity
     * @return void
     */
    public function question($string, $verbosity = null)
    {}

    /**
     * Write a string as error output.
     *
     * @param  string  $string
     * @param  'v'|'vv'|'vvv'|'quiet'|'normal'|\Symfony\Component\Console\Output\OutputInterface::VERBOSITY_*|null  $verbosity
     * @return void
     */
    public function error($string, $verbosity = null)
    {}

    /**
     * Write a string as warning output.
     *
     * @param  string  $string
     * @param  'v'|'vv'|'vvv'|'quiet'|'normal'|\Symfony\Component\Console\Output\OutputInterface::VERBOSITY_*|null  $verbosity
     * @return void
     */
    public function warn($string, $verbosity = null)
    {}

    /**
     * Write a string in an alert box.
     *
     * @param  string  $string
     * @param  'v'|'vv'|'vvv'|'quiet'|'normal'|\Symfony\Component\Console\Output\OutputInterface::VERBOSITY_*|null  $verbosity
     * @return void
     */
    public function alert($string, $verbosity = null)
    {}

    /**
     * Write a blank line.
     *
     * @param  int  $count
     * @return $this
     */
    public function newLine($count = 1)
    {}

    /**
     * Set the input interface implementation.
     *
     * @param  \Symfony\Component\Console\Input\InputInterface  $input
     * @return void
     */
    public function setInput(InputInterface $input)
    {}

    /**
     * Set the output interface implementation.
     *
     * @param  \Illuminate\Console\OutputStyle  $output
     * @return void
     */
    public function setOutput(OutputStyle $output)
    {}

    /**
     * Set the verbosity level.
     *
     * @param  'v'|'vv'|'vvv'|'quiet'|'normal'|\Symfony\Component\Console\Output\OutputInterface::VERBOSITY_*  $level
     * @return void
     */
    protected function setVerbosity($level)
    {}

    /**
     * Get the verbosity level in terms of Symfony's OutputInterface level.
     *
     * @param  'v'|'vv'|'vvv'|'quiet'|'normal'|\Symfony\Component\Console\Output\OutputInterface::VERBOSITY_*|null  $level
     * @return int
     */
    protected function parseVerbosity($level = null)
    {}

    /**
     * Get the output implementation.
     *
     * @return \Illuminate\Console\OutputStyle
     */
    public function getOutput()
    {}

    /**
     * Get the output component factory implementation.
     *
     * @return \Illuminate\Console\View\Components\Factory
     */
    public function outputComponents()
    {}
}
