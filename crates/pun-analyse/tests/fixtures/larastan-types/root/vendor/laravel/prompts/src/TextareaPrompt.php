<?php

namespace Laravel\Prompts;

use Closure;
use Laravel\Prompts\Support\Utils;
use Laravel\Prompts\Themes\Default\Concerns\InteractsWithStrings;

class TextareaPrompt extends Prompt
{
    use Concerns\Scrolling;
    use Concerns\TypedValue;
    use InteractsWithStrings;

    protected int $minWidth = 0;

    /**
     * The width of the textarea.
     */
    public int $width = 60;

    /**
     * Create a new TextareaPrompt instance.
     */
    public function __construct(
        public string $label,
        public string $placeholder = '',
        public string $default = '',
        public bool|string $required = false,
        public mixed $validate = null,
        public string $hint = '',
        int $rows = 5,
        public ?Closure $transform = null,
    ) {}

    /**
     * Get the formatted value with a virtual cursor.
     */
    public function valueWithCursor(): string
    {}

    /**
     * The word-wrapped version of the typed value.
     */
    public function wrappedValue(): string
    {}

    /**
     * The formatted lines.
     *
     * @return array<int, string>
     */
    public function lines(): array
    {}

    /**
     * The currently visible lines.
     *
     * @return array<int, string>
     */
    public function visible(): array
    {}

    /**
     * Handle the up key press.
     */
    protected function handleUpKey(): void
    {}

    /**
     * Handle the down key press.
     */
    protected function handleDownKey(): void
    {}

    /**
     * Adjust the visible window to ensure the cursor is always visible.
     */
    protected function adjustVisibleWindow(): void
    {}

    /**
     * Get the index of the current line that the cursor is on.
     */
    protected function currentLineIndex(): int
    {}

    /**
     * Calculate the cursor offset considering wrapped words.
     */
    protected function cursorOffset(): int
    {}

    /**
     * A wrapped version of the placeholder with the virtual cursor.
     */
    protected function wrappedPlaceholderWithCursor(): string
    {}
}
