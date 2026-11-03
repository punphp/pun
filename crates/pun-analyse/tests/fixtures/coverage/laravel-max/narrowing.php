<?php

namespace LaravelMax\Narrowing;

use Closure;

/**
 * @method mixed ask(string $question, string $default = null)
 */
class Factory
{
    /** @param list<mixed> $arguments */
    public function __call(string $method, array $arguments): mixed
    {
        return null;
    }
}

abstract class Prompt
{
    /** @param Closure($this): mixed $fallback */
    public static function fallbackUsing(Closure $fallback): void
    {
    }

    /** @param Closure(static): mixed $fallback */
    public static function fallbackStatic(Closure $fallback): void
    {
    }
}

class TextPrompt extends Prompt
{
}

class Narrowing
{
    public function __construct(private Factory $factory)
    {
    }

    public function magic(?string $default): void
    {
        $this->factory->ask('question', $default);
        $this->factory->ask('question', true);
    }

    public function staticCall(): void
    {
        TextPrompt::fallbackUsing(fn (TextPrompt $prompt) => '');
        TextPrompt::fallbackUsing(fn (int $prompt) => '');
        TextPrompt::fallbackStatic(fn (int $prompt) => '');
    }

    /** @param string|array<string, string>|null $column */
    public function assignedArgument($column, ?string $aggregate): void
    {
        $columns = is_string($columns = $column) ? [$column => $aggregate] : $column;

        if (! array_key_exists('key', $columns)) {
            $columns['key'] = 'MAX';
        }
    }

    public function recaller(): ?TextPrompt
    {
        return null;
    }

    public function takesPrompt(TextPrompt $prompt): void
    {
    }

    public function negatedAssignment(?TextPrompt $prompt): void
    {
        if (! is_null($recaller = $this->recaller())) {
            $this->takesPrompt($recaller);
        }

        if (! is_null($assigned = $prompt)) {
            $this->takesPrompt($prompt);
        } else {
            $this->takesPrompt($prompt);
            $this->takesPrompt($assigned);
        }
    }
}
