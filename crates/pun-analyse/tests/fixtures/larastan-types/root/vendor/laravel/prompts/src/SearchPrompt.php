<?php

namespace Laravel\Prompts;

use Closure;
use InvalidArgumentException;

class SearchPrompt extends Prompt
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
     * Create a new SearchPrompt instance.
     *
     * @param  Closure(string): array<int|string, string>  $options
     */
    public function __construct(
        public string $label,
        public Closure $options,
        public string $placeholder = '',
        public int $scroll = 5,
        public mixed $validate = null,
        public string $hint = '',
        public bool|string $required = true,
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
     * The currently visible matches.
     *
     * @return array<string>
     */
    public function visible(): array
    {}

    /**
     * Get the current search query.
     */
    public function searchValue(): string
    {}

    /**
     * Get the selected value.
     */
    public function value(): int|string|null
    {}

    /**
     * Get the selected label.
     */
    public function label(): ?string
    {}
}
