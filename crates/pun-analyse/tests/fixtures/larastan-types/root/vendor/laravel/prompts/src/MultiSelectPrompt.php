<?php

namespace Laravel\Prompts;

use Closure;
use Illuminate\Support\Collection;

class MultiSelectPrompt extends Prompt
{
    use Concerns\HasInfo;
    use Concerns\Scrolling;

    /**
     * The options for the multi-select prompt.
     *
     * @var array<int|string, string>
     */
    public array $options;

    /**
     * The default values the multi-select prompt.
     *
     * @var array<int|string>
     */
    public array $default;

    /**
     * The selected values.
     *
     * @var array<int|string>
     */
    protected array $values = [];

    /**
     * Create a new MultiSelectPrompt instance.
     *
     * @param  array<int|string, string>|Collection<int|string, string>  $options
     * @param  array<int|string>|Collection<int, int|string>  $default
     */
    public function __construct(
        public string $label,
        array|Collection $options,
        array|Collection $default = [],
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
     * Get the selected values.
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
     * The currently visible options.
     *
     * @return array<int|string, string>
     */
    public function visible(): array
    {}

    /**
     * Check whether the value is currently highlighted.
     */
    public function isHighlighted(string $value): bool
    {}

    /**
     * Check whether the value is currently selected.
     */
    public function isSelected(string $value): bool
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
}
