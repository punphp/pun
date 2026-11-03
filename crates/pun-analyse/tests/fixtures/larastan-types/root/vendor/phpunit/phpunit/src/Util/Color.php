<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Util;

use const DIRECTORY_SEPARATOR;
use const PHP_EOL;
use function array_key_exists;
use function array_map;
use function array_walk;
use function assert;
use function count;
use function explode;
use function implode;
use function max;
use function min;
use function preg_replace;
use function preg_replace_callback;
use function preg_split;
use function sprintf;
use function str_pad;
use function strlen;
use function strtr;
use function trim;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Color
{
    /**
     * @var non-empty-array<non-empty-string, non-empty-string>
     */
    private const array WHITESPACE_MAP = [
        ' '  => '·',
        "\t" => '⇥',
    ];

    /**
     * @var non-empty-array<non-empty-string, non-empty-string>
     */
    private const array WHITESPACE_EOL_MAP = [
        ' '  => '·',
        "\t" => '⇥',
        "\n" => '↵',
        "\r" => '⟵',
    ];

    /**
     * @var non-empty-array<non-empty-string, non-empty-string>
     */
    private const array ANSI_CODES = [
        'reset'      => '0',
        'bold'       => '1',
        'dim'        => '2',
        'dim-reset'  => '22',
        'underlined' => '4',
        'fg-default' => '39',
        'fg-black'   => '30',
        'fg-red'     => '31',
        'fg-green'   => '32',
        'fg-yellow'  => '33',
        'fg-blue'    => '34',
        'fg-magenta' => '35',
        'fg-cyan'    => '36',
        'fg-white'   => '37',
        'bg-default' => '49',
        'bg-black'   => '40',
        'bg-red'     => '41',
        'bg-green'   => '42',
        'bg-yellow'  => '43',
        'bg-blue'    => '44',
        'bg-magenta' => '45',
        'bg-cyan'    => '46',
        'bg-white'   => '47',
    ];

    /**
     * @param non-empty-string $color
     */
    public static function colorize(string $color, string $buffer): string
    {}

    /**
     * @param non-empty-string  $color
     * @param ?non-negative-int $columns
     *
     * @return non-empty-string
     */
    public static function colorizeTextBox(string $color, string $buffer, ?int $columns = null): string
    {}

    /**
     * @param non-empty-string  $path
     * @param ?non-empty-string $previousPath
     */
    public static function colorizePath(string $path, ?string $previousPath = null, bool $colorizeFilename = false): string
    {}

    public static function dim(string $buffer): string
    {}

    public static function visualizeWhitespace(string $buffer, bool $visualizeEOL = false): string
    {}

    private static function optimizeColor(string $buffer): string
    {}
}
