<?php

declare(strict_types=1);

namespace Dotenv\Parser;

use Dotenv\Util\Regex;

final class Lines
{
    /**
     * This class is a singleton.
     *
     * @codeCoverageIgnore
     *
     * @return void
     */
    private function __construct()
    {}

    /**
     * Process the array of lines of environment variables.
     *
     * This will produce an array of raw entries, one per variable.
     *
     * @param string[] $lines
     *
     * @return string[]
     */
    public static function process(array $lines)
    {}

    /**
     * Used to make all multiline variable process.
     *
     * @param bool     $multiline
     * @param string   $line
     * @param string[] $buffer
     *
     * @return array{bool,string, string[]}
     */
    private static function multilineProcess(bool $multiline, string $line, array $buffer)
    {}

    /**
     * Determine if the given line can be the start of a multiline variable.
     *
     * @param string $line
     *
     * @return bool
     */
    private static function looksLikeMultilineStart(string $line)
    {}

    /**
     * Determine if the given line can be the start of a multiline variable.
     *
     * @param string $line
     * @param bool   $started
     *
     * @return bool
     */
    private static function looksLikeMultilineStop(string $line, bool $started)
    {}

    /**
     * Determine if the line in the file is a comment or whitespace.
     *
     * @param string $line
     *
     * @return bool
     */
    private static function isCommentOrWhitespace(string $line)
    {}
}
