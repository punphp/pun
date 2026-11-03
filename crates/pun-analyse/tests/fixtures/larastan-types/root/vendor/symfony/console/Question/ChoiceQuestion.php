<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Question;

use Symfony\Component\Console\Exception\InvalidArgumentException;

/**
 * Represents a choice question.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class ChoiceQuestion extends Question
{
    private bool $multiselect = false;
    private string $prompt = ' > ';
    private string $errorMessage = 'Value "%s" is invalid';

    /**
     * @param string                                   $question The question to ask to the user
     * @param array<string|bool|int|float|\Stringable> $choices  The list of available choices
     * @param string|bool|int|float|null               $default  The default answer to return
     */
    public function __construct(
        string $question,
        private array $choices,
        string|bool|int|float|null $default = null,
    ) {}

    /**
     * @return array<string|bool|int|float|\Stringable>
     */
    public function getChoices(): array
    {}

    /**
     * Sets multiselect option.
     *
     * When multiselect is set to true, multiple choices can be answered.
     *
     * @return $this
     */
    public function setMultiselect(bool $multiselect): static
    {}

    /**
     * Returns whether the choices are multiselect.
     */
    public function isMultiselect(): bool
    {}

    /**
     * Gets the prompt for choices.
     */
    public function getPrompt(): string
    {}

    /**
     * Sets the prompt for choices.
     *
     * @return $this
     */
    public function setPrompt(string $prompt): static
    {}

    /**
     * Sets the error message for invalid values.
     *
     * The error message has a string placeholder (%s) for the invalid value.
     *
     * @return $this
     */
    public function setErrorMessage(string $errorMessage): static
    {}

    private function getDefaultValidator(): callable
    {}
}
