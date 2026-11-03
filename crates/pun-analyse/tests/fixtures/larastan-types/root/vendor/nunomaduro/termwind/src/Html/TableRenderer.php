<?php

declare(strict_types=1);

namespace Termwind\Html;

use Iterator;
use Symfony\Component\Console\Helper\Table;
use Symfony\Component\Console\Helper\TableCell;
use Symfony\Component\Console\Helper\TableCellStyle;
use Symfony\Component\Console\Helper\TableSeparator;
use Symfony\Component\Console\Output\BufferedOutput;
use Symfony\Component\Console\Output\OutputInterface;
use Termwind\Components\Element;
use Termwind\HtmlRenderer;
use Termwind\Termwind;
use Termwind\ValueObjects\Node;
use Termwind\ValueObjects\Styles;

/**
 * @internal
 */
final class TableRenderer
{
    /**
     * Symfony table object uses for table generation.
     */
    private Table $table;

    /**
     * This object is used for accumulating output data from Symfony table object and return it as a string.
     */
    private BufferedOutput $output;

    public function __construct()
    {}

    /**
     * Converts table output to the content element.
     */
    public function toElement(Node $node): Element
    {}

    /**
     * Looks for thead, tfoot, tbody, tr elements in a given DOM and appends rows from them to the Symfony table object.
     */
    private function parseTable(Node $node): void
    {}

    /**
     * Looks for table header title and tr elements in a given thead DOM node and adds them to the Symfony table object.
     */
    private function parseHeader(Node $node): void
    {}

    /**
     * Looks for table footer and tr elements in a given tfoot DOM node and adds them to the Symfony table object.
     */
    private function parseFoot(Node $node): void
    {}

    /**
     * Looks for tr elements in a given DOM node and adds them to the Symfony table object.
     */
    private function parseBody(Node $node): void
    {}

    /**
     * Parses table tr elements.
     */
    private function parseRows(Node $node): void
    {}

    /**
     * Looks for th, td elements in a given DOM node and converts them to a table cells.
     *
     * @return Iterator<array<int, TableCell>|TableSeparator>
     */
    private function parseRow(Node $node): Iterator
    { yield; }

    /**
     * Parses tr, td tag class attribute and passes bg, fg and options to a table cell style.
     */
    private function parseCellStyle(string $styles, string $align = TableCellStyle::DEFAULT_ALIGN): TableCellStyle
    {}

    /**
     * Get styled representation of title.
     */
    private function parseTitleStyle(Node $node): string
    {}
}
