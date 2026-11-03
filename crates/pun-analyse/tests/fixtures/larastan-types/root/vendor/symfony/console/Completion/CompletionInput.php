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

use Symfony\Component\Console\Exception\RuntimeException;
use Symfony\Component\Console\Input\ArgvInput;
use Symfony\Component\Console\Input\InputDefinition;
use Symfony\Component\Console\Input\InputOption;

/**
 * An input specialized for shell completion.
 *
 * This input allows unfinished option names or values and exposes what kind of
 * completion is expected.
 *
 * @author Wouter de Jong <wouter@wouterj.nl>
 */
final class CompletionInput extends ArgvInput
{
    public const TYPE_ARGUMENT_VALUE = 'argument_value';
    public const TYPE_OPTION_VALUE = 'option_value';
    public const TYPE_OPTION_NAME = 'option_name';
    public const TYPE_NONE = 'none';

    private array $tokens;
    private int $currentIndex;
    private string $completionType;
    private ?string $completionName = null;
    private string $completionValue = '';

    /**
     * Converts a terminal string into tokens.
     *
     * This is required for shell completions without COMP_WORDS support.
     */
    public static function fromString(string $inputStr, int $currentIndex): self
    {}

    /**
     * Create an input based on an COMP_WORDS token list.
     *
     * @param string[] $tokens       the set of split tokens (e.g. COMP_WORDS or argv)
     * @param int      $currentIndex the index of the cursor (e.g. COMP_CWORD)
     */
    public static function fromTokens(array $tokens, int $currentIndex): self
    {}

    public function bind(InputDefinition $definition): void
    {}

    /**
     * Returns the type of completion required.
     *
     * TYPE_ARGUMENT_VALUE when completing the value of an input argument
     * TYPE_OPTION_VALUE   when completing the value of an input option
     * TYPE_OPTION_NAME    when completing the name of an input option
     * TYPE_NONE           when nothing should be completed
     *
     * TYPE_OPTION_NAME and TYPE_NONE are already implemented by the Console component.
     *
     * @return self::TYPE_*
     */
    public function getCompletionType(): string
    {}

    /**
     * The name of the input option or argument when completing a value.
     *
     * @return string|null returns null when completing an option name
     */
    public function getCompletionName(): ?string
    {}

    /**
     * The value already typed by the user (or empty string).
     */
    public function getCompletionValue(): string
    {}

    public function mustSuggestOptionValuesFor(string $optionName): bool
    {}

    public function mustSuggestArgumentValuesFor(string $argumentName): bool
    {}

    protected function parseToken(string $token, bool $parseOptions): bool
    {}

    private function getOptionFromToken(string $optionToken): ?InputOption
    {}

    /**
     * The token of the cursor, or the last token if the cursor is at the end of the input.
     */
    private function getRelevantToken(): string
    {}

    /**
     * Whether the cursor is "free" (i.e. at the end of the input preceded by a space).
     */
    private function isCursorFree(): bool
    {}

    public function __toString(): string
    {}
}
