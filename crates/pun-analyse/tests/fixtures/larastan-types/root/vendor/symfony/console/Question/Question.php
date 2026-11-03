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
use Symfony\Component\Console\Exception\LogicException;
use Symfony\Component\Validator\Constraint;

/**
 * Represents a Question.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class Question
{
    private ?int $attempts = null;
    private bool $hidden = false;
    private bool $hiddenFallback = true;
    /**
     * @var (\Closure(string):list<string>)|null
     */
    private ?\Closure $autocompleterCallback = null;
    /**
     * @var (\Closure(mixed):mixed)|null
     */
    private ?\Closure $validator = null;
    /**
     * @var (\Closure(mixed):mixed)|null
     */
    private ?\Closure $normalizer = null;
    private bool $trimmable = true;
    private bool $multiline = false;
    private ?int $timeout = null;
    /**
     * @var Constraint[]
     */
    private array $constraints = [];

    /**
     * @param string                     $question The question to ask to the user
     * @param string|bool|int|float|null $default  The default answer to return if the user enters nothing.
     *                                             Object defaults set on InputArgument/InputOption are not propagated here.
     */
    public function __construct(
        private string $question,
        private string|bool|int|float|null $default = null,
    ) {}

    /**
     * Returns the question.
     */
    public function getQuestion(): string
    {}

    /**
     * Returns the default answer.
     */
    public function getDefault(): string|bool|int|float|null
    {}

    /**
     * Returns whether the user response accepts newline characters.
     */
    public function isMultiline(): bool
    {}

    /**
     * Sets whether the user response should accept newline characters.
     *
     * @return $this
     */
    public function setMultiline(bool $multiline): static
    {}

    /**
     * Returns the timeout in seconds.
     */
    public function getTimeout(): ?int
    {}

    /**
     * Sets the maximum time the user has to answer the question.
     * If the user does not answer within this time, an exception will be thrown.
     *
     * @return $this
     */
    public function setTimeout(?int $seconds): static
    {}

    /**
     * Returns whether the user response must be hidden.
     */
    public function isHidden(): bool
    {}

    /**
     * Sets whether the user response must be hidden or not.
     *
     * @return $this
     *
     * @throws LogicException In case the autocompleter is also used
     */
    public function setHidden(bool $hidden): static
    {}

    /**
     * In case the response cannot be hidden, whether to fallback on non-hidden question or not.
     */
    public function isHiddenFallback(): bool
    {}

    /**
     * Sets whether to fallback on non-hidden question if the response cannot be hidden.
     *
     * @return $this
     */
    public function setHiddenFallback(bool $fallback): static
    {}

    /**
     * Gets values for the autocompleter.
     */
    public function getAutocompleterValues(): ?iterable
    {}

    /**
     * Sets values for the autocompleter.
     *
     * @return $this
     *
     * @throws LogicException
     */
    public function setAutocompleterValues(?iterable $values): static
    {}

    /**
     * Gets the callback function used for the autocompleter.
     *
     * @return (callable(string):list<string>)|null
     */
    public function getAutocompleterCallback(): ?callable
    {}

    /**
     * Sets the callback function used for the autocompleter.
     *
     * The callback is passed the user input as argument and must return a list of corresponding suggestions.
     *
     * @param (callable(string):list<string>)|null $callback
     *
     * @return $this
     */
    public function setAutocompleterCallback(?callable $callback): static
    {}

    /**
     * Sets a validator for the question.
     *
     * @param (callable(mixed):mixed)|null $validator
     *
     * @return $this
     */
    public function setValidator(?callable $validator): static
    {}

    /**
     * Gets the validator for the question.
     *
     * @return (callable(mixed):mixed)|null
     */
    public function getValidator(): ?callable
    {}

    /**
     * Sets the maximum number of attempts.
     *
     * Null means an unlimited number of attempts.
     *
     * @return $this
     *
     * @throws InvalidArgumentException in case the number of attempts is invalid
     */
    public function setMaxAttempts(?int $attempts): static
    {}

    /**
     * Gets the maximum number of attempts.
     *
     * Null means an unlimited number of attempts.
     */
    public function getMaxAttempts(): ?int
    {}

    /**
     * Sets a normalizer for the response.
     *
     * @param callable(mixed):mixed $normalizer
     *
     * @return $this
     */
    public function setNormalizer(callable $normalizer): static
    {}

    /**
     * Gets the normalizer for the response.
     *
     * @return (callable(mixed):mixed)|null
     */
    public function getNormalizer(): ?callable
    {}

    protected function isAssoc(array $array): bool
    {}

    public function isTrimmable(): bool
    {}

    /**
     * @return $this
     */
    public function setTrimmable(bool $trimmable): static
    {}

    /**
     * @param Constraint[] $constraints
     *
     * @return $this
     */
    public function setConstraints(array $constraints): static
    {}

    /**
     * @return Constraint[]
     */
    public function getConstraints(): array
    {}
}
