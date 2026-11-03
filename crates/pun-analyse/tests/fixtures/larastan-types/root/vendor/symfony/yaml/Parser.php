<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Yaml;

use Symfony\Component\Yaml\Exception\ParseException;
use Symfony\Component\Yaml\Tag\TaggedValue;

/**
 * Parser parses YAML strings to convert them to PHP arrays.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @final
 */
class Parser
{
    public const TAG_PATTERN = '(?P<tag>![\w!.\/:-]+)';
    public const BLOCK_SCALAR_HEADER_PATTERN = '(?P<separator>\||>)(?P<modifiers>\+|\-|\d+|\+\d+|\-\d+|\d+\+|\d+\-)?(?P<comments>[ \t]+#.*)?';
    public const REFERENCE_PATTERN = '#^&(?P<ref>[^ ]++) *+(?P<value>.*)#u';
    public const DEFAULT_MAX_NESTING_LEVEL = 128;
    public const DEFAULT_MAX_ALIASES_FOR_COLLECTIONS = 128;

    private ?string $filename = null;
    private int $offset = 0;
    private int $numberOfParsedLines = 0;
    private ?int $totalNumberOfLines = null;
    private array $lines = [];
    private int $currentLineNb = -1;
    private string $currentLine = '';
    private array $refs = [];
    private array $skippedLineNumbers = [];
    private array $locallySkippedLineNumbers = [];
    private array $refsBeingParsed = [];
    private ?ParserState $state = null;

    public function __construct(int $maxNestingLevel = self::DEFAULT_MAX_NESTING_LEVEL, int $maxAliasesForCollections = self::DEFAULT_MAX_ALIASES_FOR_COLLECTIONS)
    {}

    /**
     * Parses a YAML file into a PHP value.
     *
     * @param string                     $filename The path to the YAML file to be parsed
     * @param int-mask-of<Yaml::PARSE_*> $flags    A bit field of Yaml::PARSE_* constants to customize the YAML parser behavior
     *
     * @throws ParseException If the file could not be read or the YAML is not valid
     */
    public function parseFile(string $filename, int $flags = 0): mixed
    {}

    /**
     * Parses a YAML string to a PHP value.
     *
     * @param string                     $value A YAML string
     * @param int-mask-of<Yaml::PARSE_*> $flags A bit field of Yaml::PARSE_* constants to customize the YAML parser behavior
     *
     * @throws ParseException If the YAML is not valid
     */
    public function parse(string $value, int $flags = 0): mixed
    {}

    private function getState(): ParserState
    {}

    private function doParse(string $value, int $flags): mixed
    {}

    private function parseBlock(int $offset, string $yaml, int $flags): mixed
    {}

    /**
     * Returns the current line number (takes the offset into account).
     *
     * @internal
     */
    public function getRealCurrentLineNb(): int
    {}

    private function getCurrentLineIndentation(): int
    {}

    /**
     * Returns the next embed block of YAML.
     *
     * @param int|null $indentation The indent level at which the block is to be read, or null for default
     * @param bool     $inSequence  True if the enclosing data structure is a sequence
     *
     * @throws ParseException When indentation problem are detected
     */
    private function getNextEmbedBlock(?int $indentation = null, bool $inSequence = false): string
    {}

    private function hasMoreLines(): bool
    {}

    /**
     * Moves the parser to the next line.
     */
    private function moveToNextLine(): bool
    {}

    /**
     * Moves the parser to the previous line.
     */
    private function moveToPreviousLine(): bool
    {}

    /**
     * Parses a YAML value.
     *
     * @param string $value   A YAML value
     * @param int    $flags   A bit field of Yaml::PARSE_* constants to customize the YAML parser behavior
     * @param string $context The parser context (either sequence or mapping)
     *
     * @throws ParseException When reference does not exist
     */
    private function parseValue(string $value, int $flags, string $context): mixed
    {}

    /**
     * Parses a block scalar.
     *
     * @param string $style       The style indicator that was used to begin this block scalar (| or >)
     * @param string $chomping    The chomping indicator that was used to begin this block scalar (+ or -)
     * @param int    $indentation The indentation indicator that was used to begin this block scalar
     */
    private function parseBlockScalar(string $style, string $chomping = '', int $indentation = 0): string
    {}

    /**
     * Returns true if the next line is indented.
     */
    private function isNextLineIndented(): bool
    {}

    private function isCurrentLineEmpty(): bool
    {}

    private function isCurrentLineBlank(): bool
    {}

    private function isCurrentLineComment(): bool
    {}

    private function isCurrentLineLastLineInDocument(): bool
    {}

    private function cleanup(string $value, ?bool &$hasContentOnStartMarkerLine = null): string
    {}

    /**
     * Tells whether the next line belongs to a block scalar whose header ends the given value.
     *
     * Unlike isNextLineIndented(), empty and comment-like lines are not skipped, as they can be part of the block scalar.
     */
    private function isNextLineInBlockScalar(string $value): bool
    {}

    /**
     * Turns "!tag &anchor value" into "&anchor !tag value", as both orders are allowed.
     *
     * Built-in tags are left alone: "!!str &foo" is the "&foo" string, as in flow collections.
     */
    private function moveAnchorBeforeTag(string $value): string
    {}

    private function removeLeadingLines(string $pattern, string $value): string
    {}

    private function isNextLineUnIndentedCollection(): bool
    {}

    private function isStringUnIndentedCollectionItem(): bool
    {}

    /**
     * A local wrapper for "preg_match" which will throw a ParseException if there
     * is an internal error in the PCRE engine.
     *
     * This avoids us needing to check for "false" every time PCRE is used
     * in the YAML engine
     *
     * @throws ParseException on a PCRE internal error
     *
     * @internal
     */
    public static function preg_match(string $pattern, string $subject, ?array &$matches = null, int $flags = 0, int $offset = 0): int
    {}

    /**
     * Trim the tag on top of the value.
     *
     * Prevent values such as "!foo {quz: bar}" to be considered as
     * a mapping block.
     */
    private function trimTag(string $value): string
    {}

    private function getLineTag(string $value, int $flags, bool $nextLineCheck = true): ?string
    {}

    private function lexInlineQuotedString(int &$cursor = 0): string
    {}

    private function lexUnquotedString(int &$cursor): string
    {}

    private function lexInlineAnchorOrAlias(int &$cursor): string
    {}

    private function lexInlineMapping(int &$cursor = 0, bool $consumeUntilEol = true): string
    {}

    private function lexInlineSequence(int &$cursor = 0, bool $consumeUntilEol = true): string
    {}

    private function lexInlineStructure(int &$cursor, string $closingTag, bool $consumeUntilEol = true): string
    {}

    private function doLexInlineStructure(int &$cursor, string $closingTag, bool $consumeUntilEol = true): string
    {}

    private function consumeWhitespaces(int &$cursor): bool
    {}
}
