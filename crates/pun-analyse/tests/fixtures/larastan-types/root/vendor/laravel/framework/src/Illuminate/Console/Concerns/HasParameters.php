<?php

namespace Illuminate\Console\Concerns;

use Symfony\Component\Console\Completion\CompletionInput;
use Symfony\Component\Console\Completion\CompletionSuggestions;
use Symfony\Component\Console\Completion\Suggestion;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputOption;

trait HasParameters
{
    /**
     * Specify the arguments and options on the command.
     *
     * @return void
     */
    protected function specifyParameters()
    {}

    /**
     * Get the console command arguments.
     *
     * @return (InputArgument|array{
     *    0: non-empty-string,
     *    1?: int-mask-of<InputArgument::REQUIRED|InputArgument::OPTIONAL|InputArgument::IS_ARRAY>,
     *    2?: string,
     *    3?: mixed,
     *    4?: list<string|Suggestion>|\Closure(CompletionInput, CompletionSuggestions): list<string|Suggestion>
     * })[]
     */
    protected function getArguments()
    {}

    /**
     * Get the console command options.
     *
     * @return (InputOption|array{
     *    0: non-empty-string,
     *    1?: string|non-empty-array<string>|null,
     *    2?: int-mask-of<InputOption::VALUE_*>,
     *    3?: string,
     *    4?: mixed,
     *    5?: list<string|Suggestion>|\Closure(CompletionInput, CompletionSuggestions): list<string|Suggestion>
     * })[]
     */
    protected function getOptions()
    {}
}
