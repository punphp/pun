<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Dumper;

use Symfony\Component\VarDumper\Cloner\Data;
use Symfony\Component\VarDumper\Cloner\DumperInterface;

/**
 * Abstract mechanism for dumping a Data object.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
abstract class AbstractDumper implements DataDumperInterface, DumperInterface
{
    public const DUMP_LIGHT_ARRAY = 1;
    public const DUMP_STRING_LENGTH = 2;
    public const DUMP_COMMA_SEPARATOR = 4;
    public const DUMP_TRAILING_COMMA = 8;

    /** @var callable|resource|string|null */
    public static $defaultOutput = 'php://output';

    protected string $line = '';
    /** @var callable|null */
    protected $lineDumper;
    /** @var resource|null */
    protected $outputStream;
    protected string $decimalPoint = '.';
    protected string $indentPad = '  ';

    private string $charset = '';

    /**
     * @param callable|resource|string|null $output  A line dumper callable, an opened stream or an output path, defaults to static::$defaultOutput
     * @param string|null                   $charset The default character encoding to use for non-UTF8 strings
     * @param int                           $flags   A bit field of static::DUMP_* constants to fine tune dumps representation
     */
    public function __construct(
        $output = null,
        ?string $charset = null,
        protected int $flags = 0,
    ) {}

    /**
     * Sets the output destination of the dumps.
     *
     * @param callable|resource|string|null $output A line dumper callable, an opened stream or an output path
     *
     * @return callable|resource|string|null The previous output destination
     */
    public function setOutput($output)
    {}

    /**
     * Sets the default character encoding to use for non-UTF8 strings.
     *
     * @return string The previous charset
     */
    public function setCharset(string $charset): string
    {}

    /**
     * Sets the indentation pad string.
     *
     * @param string $pad A string that will be prepended to dumped lines, repeated by nesting level
     *
     * @return string The previous indent pad
     */
    public function setIndentPad(string $pad): string
    {}

    /**
     * Dumps a Data object.
     *
     * @param callable|resource|string|true|null $output A line dumper callable, an opened stream, an output path or true to return the dump
     *
     * @return string|null The dump as string when $output is true
     */
    public function dump(Data $data, $output = null): ?string
    {}

    /**
     * Dumps the current line.
     *
     * @param int $depth The recursive depth in the dumped structure for the line being dumped,
     *                   or -1 to signal the end-of-dump to the line dumper callable
     */
    protected function dumpLine(int $depth): void
    {}

    /**
     * Generic line dumper callback.
     */
    protected function echoLine(string $line, int $depth, string $indentPad): void
    {}

    /**
     * Converts a non-UTF-8 string to UTF-8.
     */
    protected function utf8Encode(?string $s): ?string
    {}
}
