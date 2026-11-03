<?php

namespace Laravel\Prompts;

use Laravel\Prompts\Themes\Default\Concerns\InteractsWithStrings;

class Stream extends Prompt
{
    use InteractsWithStrings;

    protected int $minWidth = 0;

    protected string $message = '';

    /** @var array<int, string> */
    protected array $currentlyFading = [];

    protected int $maxWidth = 0;

    /** @var array<int, \Closure(string): string> */
    protected array $fadingOutColors = [];

    /**
     * Create a new Stream instance.
     */
    public function __construct()
    {}

    public function append(string $message): self
    {}

    public function close(): void
    {}

    /** @return array<int, string> */
    public function lines(): array
    {}

    public function prompt(): mixed
    {}

    /**
     * Get the value of the prompt.
     */
    public function value(): string
    {}

    /**
     * Get an array of closures that progressively fade text from full color to nearly invisible.
     *
     * @return array<int, \Closure(string): string>
     */
    protected function fadeOut(int $steps = 10): array
    {}
}
