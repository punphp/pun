<?php declare(strict_types=1);

namespace PhpParser;

class Comment implements \JsonSerializable {
    protected string $text;
    protected int $startLine;
    protected int $startFilePos;
    protected int $startTokenPos;
    protected int $endLine;
    protected int $endFilePos;
    protected int $endTokenPos;

    /**
     * Constructs a comment node.
     *
     * @param string $text Comment text (including comment delimiters like /*)
     * @param int $startLine Line number the comment started on
     * @param int $startFilePos File offset the comment started on
     * @param int $startTokenPos Token offset the comment started on
     */
    public function __construct(
        string $text,
        int $startLine = -1, int $startFilePos = -1, int $startTokenPos = -1,
        int $endLine = -1, int $endFilePos = -1, int $endTokenPos = -1
    ) {}

    /**
     * Gets the comment text.
     *
     * @return string The comment text (including comment delimiters like /*)
     */
    public function getText(): string {}

    /**
     * Gets the line number the comment started on.
     *
     * @return int Line number (or -1 if not available)
     * @phpstan-return -1|positive-int
     */
    public function getStartLine(): int {}

    /**
     * Gets the file offset the comment started on.
     *
     * @return int File offset (or -1 if not available)
     */
    public function getStartFilePos(): int {}

    /**
     * Gets the token offset the comment started on.
     *
     * @return int Token offset (or -1 if not available)
     */
    public function getStartTokenPos(): int {}

    /**
     * Gets the line number the comment ends on.
     *
     * @return int Line number (or -1 if not available)
     * @phpstan-return -1|positive-int
     */
    public function getEndLine(): int {}

    /**
     * Gets the file offset the comment ends on.
     *
     * @return int File offset (or -1 if not available)
     */
    public function getEndFilePos(): int {}

    /**
     * Gets the token offset the comment ends on.
     *
     * @return int Token offset (or -1 if not available)
     */
    public function getEndTokenPos(): int {}

    /**
     * Gets the comment text.
     *
     * @return string The comment text (including comment delimiters like /*)
     */
    public function __toString(): string {}

    /**
     * Gets the reformatted comment text.
     *
     * "Reformatted" here means that we try to clean up the whitespace at the
     * starts of the lines. This is necessary because we receive the comments
     * without leading whitespace on the first line, but with leading whitespace
     * on all subsequent lines.
     *
     * Additionally, this normalizes CRLF newlines to LF newlines.
     */
    public function getReformattedText(): string {}

    /**
     * Get length of shortest whitespace prefix (at the start of a line).
     *
     * If there is a line with no prefix whitespace, 0 is a valid return value.
     *
     * @param string $str String to check
     * @return int Length in characters. Tabs count as single characters.
     */
    private function getShortestWhitespacePrefixLen(string $str): int {}

    /**
     * @return array{nodeType:string, text:mixed, line:mixed, filePos:mixed}
     */
    public function jsonSerialize(): array {}
}
