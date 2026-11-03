<?php

namespace Laravel\Prompts;

use Closure;
use Illuminate\Support\Collection;

class SuggestPrompt extends Prompt
{
    use Concerns\HasInfo;
    use Concerns\Scrolling;
    use Concerns\Truncation;
    use Concerns\TypedValue;

    /**
     * The options for the suggest prompt.
     *
     * @var array<string>|Closure(string): (array<string>|Collection<int, string>)
     */
    public array|Closure $options;

    /**
     * The cache of matches.
     *
     * @var array<string>|null
     */
    protected ?array $matches = null;

    /**
     * Create a new SuggestPrompt instance.
     *
     * @param  array<string>|Collection<int, string>|Closure(string): (array<string>|Collection<int, string>)  $options
     */
    public function __construct(
        public string $label,
        array|Collection|Closure $options,
        public string $placeholder = '',
        public string $default = '',
        public int $scroll = 5,
        public bool|string $required = false,
        public mixed $validate = null,
        public string $hint = '',
        public ?Closure $transform = null,
        public string|Closure $info = '',
    ) {}

    /**
     * Get the value of the highlighted option.
     */
    public function highlightedValue(): ?string
    {}

    /**
     * Get the entered value with a virtual cursor.
     */
    public function valueWithCursor(int $maxWidth): string
    {}

    /**
     * Get options that match the input.
     *
     * @return array<string>
     */
    public function matches(): array
    {}

    /**
     * The current visible matches.
     *
     * @return array<string>
     */
    public function visible(): array
    {}

    /**
     * Select the highlighted entry.
     */
    protected function selectHighlighted(): void
    {}
}
