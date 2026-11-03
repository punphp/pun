<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Formatter;

use Symfony\Component\Console\Exception\InvalidArgumentException;
use Symfony\Component\Console\Helper\Helper;

use function Symfony\Component\String\b;

/**
 * Formatter class for console output.
 *
 * @author Konstantin Kudryashov <ever.zet@gmail.com>
 * @author Roland Franssen <franssen.roland@gmail.com>
 */
class OutputFormatter implements WrappableOutputFormatterInterface
{
    private const ANSI_PATTERN = '\\x1B(?:\\[[0-9;:]*m|\\]8;[^;]*;[^\\x1B]*\\x1B\\\\)';
    private const ANSI_REGEX = '/'.self::ANSI_PATTERN.'/';

    private array $styles = [];
    private OutputFormatterStyleStack $styleStack;
    private array $charWidths = [];

    public function __clone()
    {}

    /**
     * Escapes "<" and ">" special chars in given text.
     */
    public static function escape(string $text): string
    {}

    /**
     * Escapes trailing "\" in given text.
     *
     * @internal
     */
    public static function escapeTrailingBackslash(string $text): string
    {}

    /**
     * Initializes console output formatter.
     *
     * @param OutputFormatterStyleInterface[] $styles Array of "name => FormatterStyle" instances
     */
    public function __construct(
        private bool $decorated = false,
        array $styles = [],
    ) {}

    public function setDecorated(bool $decorated): void
    {}

    public function isDecorated(): bool
    {}

    public function setStyle(string $name, OutputFormatterStyleInterface $style): void
    {}

    public function hasStyle(string $name): bool
    {}

    public function getStyle(string $name): OutputFormatterStyleInterface
    {}

    public function format(?string $message): ?string
    {}

    public function formatAndWrap(?string $message, int $width): string
    {}

    public function getStyleStack(): OutputFormatterStyleStack
    {}

    /**
     * Tries to create new style instance from string.
     */
    private function createStyleFromString(string $string): ?OutputFormatterStyleInterface
    {}

    /**
     * Applies current style from stack to text, if must be applied.
     */
    private function applyCurrentStyle(string $text, int $width, int &$currentLineLength, bool &$lineBreak): string
    {}

    private function addLineBreaks(string $text, int $width): string
    {}

    private function substr(string $text, int $from, ?int $length = null): string
    {}

    private function getTokens(string $text): iterable
    { yield; }

    /**
     * Returns the number of columns of each visible character, or an empty array when none uses more than one column.
     *
     * @return list<int>
     */
    private function getWidths(string $text): array
    {}

    /**
     * Wraps a mask like wordwrap() does when cutting long words, but using the number of columns of each character.
     *
     * @param list<int> $widths
     */
    private function wordwrap(string $mask, int $width, array $widths): string
    {}
}
