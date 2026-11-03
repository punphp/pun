<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\SuggestPrompt;
use Laravel\Prompts\Themes\Contracts\Scrolling;

class SuggestPromptRenderer extends Renderer implements Scrolling
{
    use Concerns\DrawsBoxes;
    use Concerns\DrawsScrollbars;

    /**
     * Render the suggest prompt.
     */
    public function __invoke(SuggestPrompt $prompt): string
    {}

    /**
     * Render the value with the cursor and an arrow.
     */
    protected function valueWithCursorAndArrow(SuggestPrompt $prompt, int $maxWidth): string
    {}

    /**
     * Render a spacer to prevent jumping when the suggestions are displayed.
     */
    protected function spaceForDropdown(SuggestPrompt $prompt): self
    {}

    /**
     * Render the options.
     */
    protected function renderOptions(SuggestPrompt $prompt): string
    {}

    /**
     * The number of lines to reserve outside of the scrollable area.
     */
    public function reservedLines(): int
    {}
}
