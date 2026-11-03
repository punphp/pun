<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Completion;

use Symfony\Component\Console\Input\InputOption;

/**
 * Stores all completion suggestions for the current input.
 *
 * @author Wouter de Jong <wouter@wouterj.nl>
 */
final class CompletionSuggestions
{
    private array $valueSuggestions = [];
    private array $optionSuggestions = [];

    /**
     * Add a suggested value for an input option or argument.
     *
     * @return $this
     */
    public function suggestValue(string|Suggestion $value): static
    {}

    /**
     * Add multiple suggested values at once for an input option or argument.
     *
     * @param list<string|Suggestion> $values
     *
     * @return $this
     */
    public function suggestValues(array $values): static
    {}

    /**
     * Add a suggestion for an input option name.
     *
     * @return $this
     */
    public function suggestOption(InputOption $option): static
    {}

    /**
     * Add multiple suggestions for input option names at once.
     *
     * @param InputOption[] $options
     *
     * @return $this
     */
    public function suggestOptions(array $options): static
    {}

    /**
     * @return InputOption[]
     */
    public function getOptionSuggestions(): array
    {}

    /**
     * @return Suggestion[]
     */
    public function getValueSuggestions(): array
    {}
}
