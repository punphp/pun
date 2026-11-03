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

use Symfony\Component\ErrorHandler\ErrorRenderer\FileLinkFormatter;
use Symfony\Component\VarDumper\Cloner\Cursor;
use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * CliDumper dumps variables for command line output.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
class CliDumper extends AbstractDumper
{
    public static bool $defaultColors;
    /** @var callable|resource|string|null */
    public static $defaultOutput = 'php://stdout';

    protected bool $colors;
    protected int $maxStringWidth = 0;
    protected array $styles = [
        // See http://en.wikipedia.org/wiki/ANSI_escape_code#graphics
        'default' => '0;38;5;208',
        'num' => '1;38;5;38',
        'const' => '1;38;5;208',
        'virtual' => '3',
        'str' => '1;38;5;113',
        'note' => '38;5;38',
        'ref' => '38;5;247',
        'public' => '39',
        'protected' => '39',
        'private' => '39',
        'meta' => '38;5;170',
        'key' => '38;5;113',
        'index' => '38;5;38',
    ];

    protected static string $controlCharsRx = '/[\x00-\x1F\x7F]+/';
    protected static array $controlCharsMap = [
        "\t" => '\t',
        "\n" => '\n',
        "\v" => '\v',
        "\f" => '\f',
        "\r" => '\r',
        "\033" => '\e',
    ];
    protected static string $unicodeCharsRx = "/[\u{0080}-\u{009F}\u{00A0}\u{00AD}\u{034F}\u{061C}\u{115F}\u{1160}\u{17B4}\u{17B5}\u{180E}\u{2000}-\u{200F}\u{202F}\u{205F}\u{2060}-\u{2064}\u{206A}-\u{206F}\u{3000}\u{2800}\u{3164}\u{FEFF}\u{FFA0}\u{1D159}\u{1D173}-\u{1D17A}]/u";

    protected bool $collapseNextHash = false;
    protected bool $expandNextHash = false;

    private array $displayOptions = [
        'fileLinkFormat' => null,
    ];

    private bool $handlesHrefGracefully;

    public function __construct($output = null, ?string $charset = null, int $flags = 0)
    {}

    /**
     * Enables/disables colored output.
     */
    public function setColors(bool $colors): void
    {}

    /**
     * Sets the maximum number of characters per line for dumped strings.
     */
    public function setMaxStringWidth(int $maxStringWidth): void
    {}

    /**
     * Configures styles.
     *
     * @param array $styles A map of style names to style definitions
     */
    public function setStyles(array $styles): void
    {}

    /**
     * Configures display options.
     *
     * @param array $displayOptions A map of display options to customize the behavior
     */
    public function setDisplayOptions(array $displayOptions): void
    {}

    public function dumpScalar(Cursor $cursor, string $type, string|int|float|bool|null $value): void
    {}

    public function dumpString(Cursor $cursor, string $str, bool $bin, int $cut): void
    {}

    public function enterHash(Cursor $cursor, int $type, string|int|null $class, bool $hasChild): void
    {}

    public function leaveHash(Cursor $cursor, int $type, string|int|null $class, bool $hasChild, int $cut): void
    {}

    /**
     * Dumps an ellipsis for cut children.
     *
     * @param bool $hasChild When the dump of the hash has child item
     * @param int  $cut      The number of items the hash has been cut by
     */
    protected function dumpEllipsis(Cursor $cursor, bool $hasChild, int $cut): void
    {}

    /**
     * Dumps a key in a hash structure.
     */
    protected function dumpKey(Cursor $cursor): void
    {}

    /**
     * Decorates a value with some style.
     *
     * @param string $style The type of style being applied
     * @param string $value The value being styled
     * @param array  $attr  Optional context information
     */
    protected function style(string $style, string $value, array $attr = []): string
    {}

    protected function supportsColors(): bool
    {}

    protected function dumpLine(int $depth, bool $endOfValue = false): void
    {}

    protected function endValue(Cursor $cursor): void
    {}

    /**
     * Returns the stream to probe for color support: on the CLI, php://output is backed by STDOUT.
     */
    private function getColorSupportStream(): mixed
    {}

    /**
     * Returns true if the stream supports colorization.
     *
     * Reference: Composer\XdebugHandler\Process::supportsColor
     * https://github.com/composer/xdebug-handler
     */
    private function hasColorSupport(mixed $stream): bool
    {}

    /**
     * Returns true if the Windows terminal supports true color.
     *
     * Note that this does not check an output stream, but relies on environment
     * variables from known implementations, or a PHP and Windows version that
     * supports true color.
     */
    private function isWindowsTrueColor(): bool
    {}

    private function getSourceLink(string $file, int $line): string|false
    {}
}
