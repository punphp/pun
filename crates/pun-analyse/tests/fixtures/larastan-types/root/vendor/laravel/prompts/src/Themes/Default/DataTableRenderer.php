<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\DataTablePrompt;
use Laravel\Prompts\Themes\Contracts\Scrolling;
use Laravel\Prompts\Themes\Default\Concerns\DrawsBoxes;
use Laravel\Prompts\Themes\Default\Concerns\DrawsScrollbars;

class DataTableRenderer extends Renderer implements Scrolling
{
    use DrawsBoxes;
    use DrawsScrollbars;

    /**
     * Render the data table.
     */
    public function __invoke(DataTablePrompt $prompt): string
    {}

    /**
     * Render the submit state.
     */
    protected function renderSubmit(DataTablePrompt $prompt, int $maxWidth): string
    {}

    /**
     * Render the cancel state.
     */
    protected function renderCancel(DataTablePrompt $prompt, int $maxWidth): string
    {}

    /**
     * Render the active/browse/search state.
     */
    protected function renderActive(DataTablePrompt $prompt, int $maxWidth): string
    {}

    /**
     * Render a column-aware border line.
     *
     * @param  array<int, int>  $widths
     */
    protected function renderBorder(string $left, string $mid, string $right, array $widths, string $color = 'gray'): string
    {}

    /**
     * Render a simple border line without column separators.
     */
    protected function renderSimpleBorder(string $left, string $right, int $innerWidth, string $color = 'gray'): string
    {}

    /**
     * Render the search line content.
     */
    protected function renderSearchLine(DataTablePrompt $prompt, int $maxWidth): string
    {}

    /**
     * Render data rows with scrollbar support.
     *
     * @param  array<int|string, array<int, string>>  $filtered
     * @param  array<int|string, array<int, string>>  $visible
     * @param  array<int, int>  $widths
     * @return array<int, string>
     */
    protected function renderDataRows(DataTablePrompt $prompt, array $filtered, array $visible, array $widths, int $numCols, int $innerWidth, bool $strikethrough = false): array
    {}

    /**
     * Compute column widths that fit within maxWidth.
     *
     * Columns get their natural (P85) width. Only shrink proportionally
     * if the total exceeds available terminal space.
     *
     * @param  array<int, string|array<int, string>>  $headers
     * @param  array<int|string, array<int, string>>  $allRows
     * @return array<int, int>
     */
    protected function computeColumnWidths(array $headers, array $allRows, int $numCols, int $maxWidth): array
    {}

    /**
     * The number of lines to reserve outside of the scrollable area.
     */
    public function reservedLines(): int
    {}
}
