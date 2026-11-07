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

use Symfony\Component\Yaml\Exception\DumpException;
use Symfony\Component\Yaml\Exception\ParseException;
use Symfony\Component\Yaml\Tag\TaggedValue;

/**
 * Inline implements a YAML parser/dumper for the YAML inline syntax.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @internal
 */
class Inline
{
    public const REGEX_QUOTED_STRING = '(?:"([^"\\\\]*+(?:\\\\.[^"\\\\]*+)*+)"|\'([^\']*+(?:\'\'[^\']*+)*+)\')';

    public static int $parsedLineNumber = -1;
    public static ?string $parsedFilename = null;

    private static bool $exceptionOnInvalidType = false;
    private static bool $objectSupport = false;
    private static bool $objectForMap = false;
    private static bool $constantSupport = false;

    public static function initialize(int $flags, ?int $parsedLineNumber = null, ?string $parsedFilename = null): void
    {}

    /**
     * Converts a YAML string to a PHP value.
     *
     * @param int   $flags      A bit field of Yaml::PARSE_* constants to customize the YAML parser behavior
     * @param array $references Mapping of variable names to values
     *
     * @throws ParseException
     */
    public static function parse(string $value, int $flags = 0, array &$references = [], ?ParserState $state = null): mixed
    {}

    /**
     * Dumps a given PHP variable to a YAML string.
     *
     * @param mixed $value The PHP variable to convert
     * @param int   $flags A bit field of Yaml::DUMP_* constants to customize the dumped YAML string
     *
     * @throws DumpException When trying to dump PHP resource
     */
    public static function dump(mixed $value, int $flags = 0, bool $rootLevel = false): string
    {}

    /**
     * Check if given array is hash or just normal indexed array.
     */
    public static function isHash(array|\ArrayObject|\stdClass $value): bool
    {}

    /**
     * Dumps a PHP array to a YAML string.
     *
     * @param array $value The PHP array to dump
     * @param int   $flags A bit field of Yaml::DUMP_* constants to customize the dumped YAML string
     */
    private static function dumpArray(array $value, int $flags): string
    {}

    /**
     * Dumps hash array to a YAML string.
     *
     * @param array|\ArrayObject|\stdClass $value The hash array to dump
     * @param int                          $flags A bit field of Yaml::DUMP_* constants to customize the dumped YAML string
     */
    private static function dumpHashArray(array|\ArrayObject|\stdClass $value, int $flags): string
    {}

    private static function dumpNull(int $flags, bool $rootLevel = false): string
    {}

    /**
     * Parses a YAML scalar.
     *
     * @throws ParseException When malformed inline YAML string is parsed
     */
    public static function parseScalar(string $scalar, int $flags = 0, ?array $delimiters = null, int &$i = 0, bool $evaluate = true, array &$references = [], ?bool &$isQuoted = null, ?ParserState $state = null): mixed
    {}

    /**
     * Parses a YAML quoted scalar.
     *
     * @throws ParseException When malformed inline YAML string is parsed
     */
    private static function parseQuotedScalar(string $scalar, int &$i = 0): string
    {}

    /**
     * Parses a YAML sequence.
     *
     * @throws ParseException When malformed inline YAML string is parsed
     */
    private static function parseSequence(ParserState $state, string $sequence, int $flags, int &$i = 0, array &$references = []): array
    {}

    /**
     * Returns the offset of the "," or "]" that ends the embedded mapping starting at $i.
     *
     * Tokens are split the same way as in Parser::lexInlineStructure().
     */
    private static function findEmbeddedMappingEnd(string $sequence, int $i): int
    {}

    /**
     * Parses a YAML mapping.
     *
     * @throws ParseException When malformed inline YAML string is parsed
     */
    private static function parseMapping(ParserState $state, string $mapping, int $flags, int &$i = 0, array &$references = []): array|\stdClass
    {}

    /**
     * Evaluates scalars and replaces magic values.
     *
     * @throws ParseException when object parsing support was disabled and the parser detected a PHP object or when a reference could not be resolved
     */
    private static function evaluateScalar(ParserState $state, string $scalar, int $flags, array &$references = [], ?bool &$isQuotedString = null): mixed
    {}

    private static function parseAnchor(string $value, int &$i): ?string
    {}

    private static function parseTag(string $value, int &$i, int $flags): ?string
    {}

    public static function evaluateBinaryScalar(string $scalar): string
    {}

    private static function isBinaryString(string $value): bool
    {}

    /**
     * Gets a regex that matches a YAML date.
     *
     * @see http://www.yaml.org/spec/1.2/spec.html#id2761573
     */
    private static function getTimestampRegex(): string
    {}

    /**
     * Gets a regex that matches a YAML number in hexadecimal notation.
     */
    private static function getHexRegex(): string
    {}
}
