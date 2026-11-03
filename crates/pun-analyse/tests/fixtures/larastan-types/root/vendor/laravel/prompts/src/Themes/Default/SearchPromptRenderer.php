<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\SearchPrompt;
use Laravel\Prompts\Themes\Contracts\Scrolling;

class SearchPromptRenderer extends Renderer implements Scrolling
{
    use Concerns\DrawsBoxes;
    use Concerns\DrawsScrollbars;

    /**
     * Render the suggest prompt.
     */
    public function __invoke(SearchPrompt $prompt): string
    {}

    /**
     * Render the value with the cursor and a search icon.
     */
    protected function valueWithCursorAndSearchIcon(SearchPrompt $prompt, int $maxWidth): string
    {}

    /**
     * Render a spacer to prevent jumping when the suggestions are displayed.
     */
    protected function spaceForDropdown(SearchPrompt $prompt): self
    {}

    /**
     * Render the options.
     */
    protected function renderOptions(SearchPrompt $prompt): string
    {}

    /**
     * The number of lines to reserve outside of the scrollable area.
     */
    public function reservedLines(): int
    {}
}
