<?php

namespace Illuminate\Console\Concerns;

use Illuminate\Console\PromptValidationException;
use Laravel\Prompts\ConfirmPrompt;
use Laravel\Prompts\MultiSearchPrompt;
use Laravel\Prompts\MultiSelectPrompt;
use Laravel\Prompts\NumberPrompt;
use Laravel\Prompts\PasswordPrompt;
use Laravel\Prompts\PausePrompt;
use Laravel\Prompts\Prompt;
use Laravel\Prompts\SearchPrompt;
use Laravel\Prompts\SelectPrompt;
use Laravel\Prompts\SuggestPrompt;
use Laravel\Prompts\TextareaPrompt;
use Laravel\Prompts\TextPrompt;
use stdClass;
use Symfony\Component\Console\Input\InputInterface;

trait ConfiguresPrompts
{
    /**
     * Configure the prompt fallbacks.
     *
     * @param  \Symfony\Component\Console\Input\InputInterface  $input
     * @return void
     */
    protected function configurePrompts(InputInterface $input)
    {}

    /**
     * Prompt the user until the given validation callback passes.
     *
     * @template PResult
     *
     * @param  \Closure(): PResult  $prompt
     * @param  bool|string  $required
     * @param  (\Closure(PResult): mixed)|null  $validate
     * @return PResult
     *
     * @throws \Illuminate\Console\PromptValidationException
     */
    protected function promptUntilValid($prompt, $required, $validate)
    {}

    /**
     * Validate the given prompt value using the validator.
     *
     * @param  mixed  $value
     * @param  mixed  $rules
     * @return ?string
     */
    protected function validatePrompt($value, $rules)
    {}

    /**
     * Get the validator instance that should be used to validate prompts.
     *
     * @param  mixed  $field
     * @param  mixed  $value
     * @param  mixed  $rules
     * @param  array  $messages
     * @param  array  $attributes
     * @return \Illuminate\Validation\Validator
     */
    protected function getPromptValidatorInstance($field, $value, $rules, array $messages = [], array $attributes = [])
    {}

    /**
     * Get the validation messages that should be used during prompt validation.
     *
     * @return array<string, string>
     */
    protected function validationMessages()
    {}

    /**
     * Get the validation attributes that should be used during prompt validation.
     *
     * @return array<string, string>
     */
    protected function validationAttributes()
    {}

    /**
     * Restore the prompts output.
     *
     * @return void
     */
    protected function restorePrompts()
    {}

    /**
     * Select fallback.
     *
     * @param  string  $label
     * @param  array<array-key, string>  $options
     * @param  string|int|null  $default
     * @return string|int
     */
    private function selectFallback($label, $options, $default = null)
    {}

    /**
     * Multi-select fallback.
     *
     * @param  string  $label
     * @param  array  $options
     * @param  array  $default
     * @param  bool|string  $required
     * @return array
     */
    private function multiselectFallback($label, $options, $default = [], $required = false)
    {}
}
