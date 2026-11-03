<?php

namespace Laravel\Prompts;

use Closure;
use Laravel\Prompts\Support\Utils;

class MultiSearchPrompt extends Prompt
{
    use Concerns\HasInfo;
    use Concerns\Scrolling;
    use Concerns\Truncation;
    use Concerns\TypedValue;

    /**
     * The cached matches.
     *
     * @var array<int|string, string>|null
     */
    protected ?array $matches = null;

    /**
     * Whether the matches are initially a list.
     */
    protected bool $isList;

    /**
     * The selected values.
     *
     * @var array<int|string, string>
     */
    public array $values = [];

    /**
     * Create a new MultiSearchPrompt instance.
     *
     * @param  Closure(string): array<int|string, string>  $options
     */
    public function __construct(
        public string $label,
        public Closure $options,
        public string $placeholder = '',
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
    public function highlightedValue(): int|string|null
    {}

    /**
     * Perform the search.
     */
    protected function search(): void
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
     * The currently visible matches
     *
     * @return array<string>
     */
    public function visible(): array
    {}

    /**
     * Toggle all options.
     */
    protected function toggleAll(): void
    {}

    /**
     * Toggle the highlighted entry.
     */
    protected function toggleHighlighted(): void
    {}

    /**
     * Get the current search query.
     */
    public function searchValue(): string
    {}

    /**
     * Get the selected value.
     *
     * @return array<int|string>
     */
    public function value(): array
    {}

    /**
     * Get the selected labels.
     *
     * @return array<string>
     */
    public function labels(): array
    {}

    /**
     * Whether the matches are initially a list.
     */
    public function isList(): bool
    {}
}
