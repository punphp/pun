<?php

namespace Laravel\Prompts\Concerns;

use Laravel\Prompts\Themes\Contracts\Scrolling as ScrollingRenderer;

trait Scrolling
{
    /**
     * The number of items to display before scrolling.
     */
    public int $scroll;

    /**
     * The index of the highlighted option.
     */
    public ?int $highlighted;

    /**
     * The index of the first visible option.
     */
    public int $firstVisible = 0;

    /**
     * Initialize scrolling.
     */
    protected function initializeScrolling(?int $highlighted = null): void
    {}

    /**
     * Reduce the scroll property to fit the terminal height.
     */
    protected function reduceScrollingToFitTerminal(): void
    {}

    /**
     * Highlight the given index.
     */
    protected function highlight(?int $index): void
    {}

    /**
     * Highlight the previous entry, or wrap around to the last entry.
     */
    protected function highlightPrevious(int $total, bool $allowNull = false): void
    {}

    /**
     * Highlight the next entry, or wrap around to the first entry.
     */
    protected function highlightNext(int $total, bool $allowNull = false): void
    {}

    /**
     * Center the highlighted option.
     */
    protected function scrollToHighlighted(int $total): void
    {}
}
