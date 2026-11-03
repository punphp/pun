<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console;

use Symfony\Component\Console\Output\AnsiColorMode;

class Terminal
{
    public const DEFAULT_COLOR_MODE = AnsiColorMode::Ansi4;

    private static ?AnsiColorMode $colorMode = null;
    private static ?int $width = null;
    private static ?int $height = null;
    private static ?bool $stty = null;
    private static ?bool $kittyGraphics = null;
    private static ?bool $iterm2Images = null;

    /**
     * About Ansi color types: https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
     * For more information about true color support with terminals https://github.com/termstandard/colors/.
     */
    public static function getColorMode(): AnsiColorMode
    {}

    /**
     * Force a terminal color mode rendering.
     */
    public static function setColorMode(?AnsiColorMode $colorMode): void
    {}

    public function getWidth(): int
    {}

    public function getHeight(): int
    {}

    /**
     * @internal
     */
    public static function hasSttyAvailable(): bool
    {}

    public static function supportsKittyGraphics(): bool
    {}

    public static function supportsITerm2Images(): bool
    {}

    public static function supportsImageProtocol(): bool
    {}

    public static function setKittyGraphicsSupport(?bool $supported): void
    {}

    public static function setITerm2ImagesSupport(?bool $supported): void
    {}

    private static function initDimensions(): void
    {}

    private static function initDimensionsUsingStty(): void
    {}

    /**
     * Runs and parses mode CON if it's available, suppressing any error output.
     *
     * @return int[]|null An array composed of the width and the height or null if it could not be parsed
     */
    private static function getConsoleMode(): ?array
    {}

    private static function getSttyColumns(): ?string
    {}

    private static function readFromProcess(string|array $command): ?string
    {}
}
