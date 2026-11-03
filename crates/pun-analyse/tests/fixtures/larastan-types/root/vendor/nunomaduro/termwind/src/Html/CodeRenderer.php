<?php

declare(strict_types=1);

namespace Termwind\Html;

use Termwind\Components\Element;
use Termwind\Termwind;
use Termwind\ValueObjects\Node;

/**
 * @internal
 */
final class CodeRenderer
{
    public const TOKEN_DEFAULT = 'token_default';

    public const TOKEN_COMMENT = 'token_comment';

    public const TOKEN_STRING = 'token_string';

    public const TOKEN_HTML = 'token_html';

    public const TOKEN_KEYWORD = 'token_keyword';

    public const ACTUAL_LINE_MARK = 'actual_line_mark';

    public const LINE_NUMBER = 'line_number';

    private const ARROW_SYMBOL_UTF8 = '➜';

    private const DELIMITER_UTF8 = '▕ '; // '▶';

    private const LINE_NUMBER_DIVIDER = 'line_divider';

    private const MARKED_LINE_NUMBER = 'marked_line';

    private const WIDTH = 3;

    /**
     * Holds the theme.
     *
     * @var array<string, string>
     */
    private const THEME = [
        self::TOKEN_STRING => 'text-gray',
        self::TOKEN_COMMENT => 'text-gray italic',
        self::TOKEN_KEYWORD => 'text-magenta strong',
        self::TOKEN_DEFAULT => 'strong',
        self::TOKEN_HTML => 'text-blue strong',

        self::ACTUAL_LINE_MARK => 'text-red strong',
        self::LINE_NUMBER => 'text-gray',
        self::MARKED_LINE_NUMBER => 'italic strong',
        self::LINE_NUMBER_DIVIDER => 'text-gray',
    ];

    private string $delimiter = self::DELIMITER_UTF8;

    private string $arrow = self::ARROW_SYMBOL_UTF8;

    private const NO_MARK = '    ';

    /**
     * Highlights HTML content from a given node and converts to the content element.
     */
    public function toElement(Node $node): Element
    {}

    /**
     * Finds extra spaces which should be removed from HTML.
     *
     * @param  array<int, string>  $lines
     */
    private function findExtraSpaces(array $lines): string
    {}

    /**
     * Returns content split into lines with numbers.
     *
     * @return array<int, array<int, array{0: string, 1: non-empty-string}>>
     */
    private function getHighlightedLines(string $source, int $startLine): array
    {}

    /**
     * Splits content into tokens.
     *
     * @return array<int, array{0: string, 1: string}>
     */
    private function tokenize(string $source): array
    {}

    /**
     * Splits tokens into lines.
     *
     * @param  array<int, array{0: string, 1: string}>  $tokens
     * @return array<int, array<int, array{0: string, 1: non-empty-string}>>
     */
    private function splitToLines(array $tokens, int $startLine): array
    {}

    /**
     * Applies colors to tokens according to a color schema.
     *
     * @param  array<int, array<int, array{0: string, 1: non-empty-string}>>  $tokenLines
     * @return array<int, string>
     */
    private function colorLines(array $tokenLines): array
    {}

    /**
     * Prepends line numbers into lines.
     *
     * @param  array<int, string>  $lines
     */
    private function lineNumbers(array $lines, int $markLine): string
    {}

    /**
     * Formats line number and applies color according to a color schema.
     */
    private function coloredLineNumber(string $token, int $lineNumber, int $length): string
    {}

    /**
     * Formats string and applies color according to a color schema.
     */
    private function styleToken(string $token, string $string): string
    {}
}
