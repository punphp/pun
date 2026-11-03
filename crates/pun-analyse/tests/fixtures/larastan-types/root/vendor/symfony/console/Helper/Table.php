<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Helper;

use Symfony\Component\Console\Exception\InvalidArgumentException;
use Symfony\Component\Console\Exception\RuntimeException;
use Symfony\Component\Console\Formatter\OutputFormatter;
use Symfony\Component\Console\Formatter\OutputFormatterInterface;
use Symfony\Component\Console\Formatter\WrappableOutputFormatterInterface;
use Symfony\Component\Console\Output\BufferedOutput;
use Symfony\Component\Console\Output\ConsoleSectionOutput;
use Symfony\Component\Console\Output\OutputInterface;

/**
 * Provides helpers to display a table.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Саша Стаменковић <umpirsky@gmail.com>
 * @author Abdellatif Ait boudad <a.aitboudad@gmail.com>
 * @author Max Grigorian <maxakawizard@gmail.com>
 * @author Dany Maillard <danymaillard93b@gmail.com>
 */
class Table
{
    private const SEPARATOR_TOP = 0;
    private const SEPARATOR_TOP_BOTTOM = 1;
    private const SEPARATOR_MID = 2;
    private const SEPARATOR_BOTTOM = 3;
    private const BORDER_OUTSIDE = 0;
    private const BORDER_INSIDE = 1;
    private const DISPLAY_ORIENTATION_DEFAULT = 'default';
    private const DISPLAY_ORIENTATION_HORIZONTAL = 'horizontal';
    private const DISPLAY_ORIENTATION_VERTICAL = 'vertical';

    private ?string $headerTitle = null;
    private ?string $footerTitle = null;
    private array $headers = [];
    private array $rows = [];
    private array $effectiveColumnWidths = [];
    private int $numberOfColumns;
    private TableStyle $style;
    private array $columnStyles = [];
    private array $columnWidths = [];
    private array $configuredColumnWidths = [];
    private array $columnMaxWidths = [];
    private array $renderedLines = [];
    private array $tagStyles = [];
    private ?\WeakMap $rowspanLines = null;
    private string $displayOrientation = self::DISPLAY_ORIENTATION_DEFAULT;

    private static array $styles;

    public function __construct(
        private OutputInterface $output,
    ) {}

    /**
     * Sets a style definition.
     */
    public static function setStyleDefinition(string $name, TableStyle $style): void
    {}

    /**
     * Gets a style definition by name.
     */
    public static function getStyleDefinition(string $name): TableStyle
    {}

    /**
     * Sets table style.
     *
     * @return $this
     */
    public function setStyle(TableStyle|string $name): static
    {}

    /**
     * Gets the current table style.
     */
    public function getStyle(): TableStyle
    {}

    /**
     * Sets table column style.
     *
     * @param TableStyle|string $name The style name or a TableStyle instance
     *
     * @return $this
     */
    public function setColumnStyle(int $columnIndex, TableStyle|string $name): static
    {}

    /**
     * Gets the current style for a column.
     *
     * If style was not set, it returns the global table style.
     */
    public function getColumnStyle(int $columnIndex): TableStyle
    {}

    /**
     * Sets the minimum width of a column.
     *
     * @return $this
     */
    public function setColumnWidth(int $columnIndex, int $width): static
    {}

    /**
     * Sets the minimum width of all columns.
     *
     * @return $this
     */
    public function setColumnWidths(array $widths): static
    {}

    /**
     * Sets the maximum width of a column.
     *
     * Any cell within this column which contents exceeds the specified width will be wrapped into multiple lines, while
     * formatted strings are preserved.
     *
     * @return $this
     */
    public function setColumnMaxWidth(int $columnIndex, int $width): static
    {}

    /**
     * @return $this
     */
    public function setHeaders(array $headers): static
    {}

    /**
     * @return $this
     */
    public function setRows(array $rows): static
    {}

    /**
     * @return $this
     */
    public function addRows(array $rows): static
    {}

    /**
     * @return $this
     */
    public function addRow(TableSeparator|array $row): static
    {}

    /**
     * Adds a row to the table, and re-renders the table.
     *
     * @return $this
     */
    public function appendRow(TableSeparator|array $row): static
    {}

    /**
     * @return $this
     */
    public function setRow(int|string $column, array $row): static
    {}

    /**
     * @return $this
     */
    public function setHeaderTitle(?string $title): static
    {}

    /**
     * @return $this
     */
    public function setFooterTitle(?string $title): static
    {}

    /**
     * @return $this
     */
    public function setHorizontal(bool $horizontal = true): static
    {}

    /**
     * @return $this
     */
    public function setVertical(bool $vertical = true): static
    {}

    /**
     * Renders table to output.
     *
     * Example:
     *
     *     +---------------+-----------------------+------------------+
     *     | ISBN          | Title                 | Author           |
     *     +---------------+-----------------------+------------------+
     *     | 99921-58-10-7 | Divine Comedy         | Dante Alighieri  |
     *     | 9971-5-0210-0 | A Tale of Two Cities  | Charles Dickens  |
     *     | 960-425-059-0 | The Lord of the Rings | J. R. R. Tolkien |
     *     +---------------+-----------------------+------------------+
     */
    public function render(): void
    {}

    /**
     * Renders horizontal header separator.
     *
     * Example:
     *
     *     +-----+-----------+-------+
     */
    private function renderRowSeparator(int $type = self::SEPARATOR_MID, ?string $title = null, ?string $titleFormat = null, ?array $rowAbove = null, ?array $rowBelow = null): void
    {}

    /**
     * Renders vertical column separator.
     */
    private function renderColumnSeparator(int $type = self::BORDER_OUTSIDE): string
    {}

    /**
     * Renders table row.
     *
     * Example:
     *
     *     | 9971-5-0210-0 | A Tale of Two Cities  | Charles Dickens  |
     */
    private function renderRow(array $row, string $cellFormat, ?string $firstCellFormat = null): void
    {}

    /**
     * Renders table cell with padding.
     */
    private function renderCell(array $row, int $column, string $cellFormat): string
    {}

    /**
     * Calculate number of columns for this table.
     */
    private function calculateNumberOfColumns(array $rows): void
    {}

    private function buildTableRows(array $rows): TableRows
    {}

    /**
     * fill rows that contains rowspan > 1.
     *
     * @throws InvalidArgumentException
     */
    private function fillNextRows(array $rows, int $line): array
    {}

    /**
     * fill cells for a row that contains colspan > 1.
     */
    private function fillCells(iterable $row): iterable
    {}

    private function copyRow(array $rows, int $line): array
    {}

    /**
     * Closes the styles that a cell leaves open at its end.
     *
     * With $eol, closes the styles that span a line break at the end of each line, and reopens them on the next line.
     */
    private function closeStyles(mixed $cell, ?string $eol = null): mixed
    {}

    /**
     * Gets number of columns by row.
     */
    private function getNumberOfColumns(array $row): int
    {}

    /**
     * Gets list of columns for the given row.
     */
    private function getRowColumns(array $row): array
    {}

    /**
     * Calculates columns widths.
     */
    private function calculateColumnsWidth(iterable $groups): void
    {}

    private function getColumnSeparatorWidth(): int
    {}

    private function getCellWidth(array $row, int $column, OutputFormatterInterface $formatter): int
    {}

    /**
     * Called after rendering to cleanup cache data.
     */
    private function cleanup(): void
    {}

    /**
     * @return array<string, TableStyle>
     */
    private static function initStyles(): array
    {}

    private function resolveStyle(TableStyle|string $name): TableStyle
    {}

    /**
     * @return string[]
     */
    private function renderLines(): array
    {}
}
