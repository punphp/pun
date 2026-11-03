<?php

namespace Laravel\Prompts;

use Closure;
use Illuminate\Support\Collection;

class AutoCompletePrompt extends Prompt
{
    use Concerns\TypedValue;

    /**
     * The options for the autocomplete prompt.
     *
     * @var array<string>|Closure(string): (array<string>|Collection<int, string>)
     */
    public array|Closure $options;

    protected string $match = '';

    protected int $highlighted = 0;

    /**
     * @var array<string>|null
     */
    protected ?array $matches = null;

    /**
     * Create a new AutoCompletePrompt instance.
     *
     * @param  array<string>|Collection<int, string>|Closure(string): (array<string>|Collection<int, string>)  $options
     */
    public function __construct(
        public string $label,
        array|Collection|Closure $options = [],
        public string $placeholder = '',
        public string $default = '',
        public bool|string $required = false,
        public mixed $validate = null,
        public string $hint = '',
        public ?Closure $transform = null,
    ) {}

    /**
     * Get the entered value with a virtual cursor.
     */
    public function valueWithCursor(int $maxWidth): string
    {}

    /**
     * Get the current matches for the typed value.
     *
     * @return array<string>
     */
    public function matches(): array
    {}

    /**
     * Get the current match.
     */
    protected function getMatch(): string
    {}
}
