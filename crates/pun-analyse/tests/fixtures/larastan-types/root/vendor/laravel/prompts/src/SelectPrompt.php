<?php

namespace Laravel\Prompts;

use Closure;
use Illuminate\Support\Collection;
use InvalidArgumentException;

class SelectPrompt extends Prompt
{
    use Concerns\HasInfo;
    use Concerns\Scrolling;

    /**
     * The options for the select prompt.
     *
     * @var array<int|string, string>
     */
    public array $options;

    /**
     * Create a new SelectPrompt instance.
     *
     * @param  array<int|string, string>|Collection<int|string, string>  $options
     */
    public function __construct(
        public string $label,
        array|Collection $options,
        public int|string|null $default = null,
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
     * Get the selected value.
     */
    public function value(): int|string|null
    {}

    /**
     * Get the selected label.
     */
    public function label(): ?string
    {}

    /**
     * The currently visible options.
     *
     * @return array<int|string, string>
     */
    public function visible(): array
    {}

    /**
     * Determine whether the given value is invalid when the prompt is required.
     */
    protected function isInvalidWhenRequired(mixed $value): bool
    {}
}
