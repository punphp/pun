<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Output;

use Symfony\Component\Console\Exception\InvalidArgumentException;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Julien Boudry <julien@condorcet.vote>
 */
enum AnsiColorMode
{
    /*
     * Classical 4-bit Ansi colors, including 8 classical colors and 8 bright color. Output syntax is "ESC[${foreGroundColorcode};${backGroundColorcode}m"
     * Must be compatible with all terminals and it's the minimal version supported.
     */
    case Ansi4;

    /*
     * 8-bit Ansi colors (240 different colors + 16 duplicate color codes, ensuring backward compatibility).
     * Output syntax is: "ESC[38;5;${foreGroundColorcode};48;5;${backGroundColorcode}m"
     * Should be compatible with most terminals.
     */
    case Ansi8;

    /*
     * 24-bit Ansi colors (RGB).
     * Output syntax is: "ESC[38;2;${foreGroundColorcodeRed};${foreGroundColorcodeGreen};${foreGroundColorcodeBlue};48;2;${backGroundColorcodeRed};${backGroundColorcodeGreen};${backGroundColorcodeBlue}m"
     * May be compatible with many modern terminals.
     */
    case Ansi24;

    /**
     * Converts an RGB hexadecimal color to the corresponding Ansi code.
     */
    public function convertFromHexToAnsiColorCode(string $hexColor): string
    {}

    private function convertFromRGB(int $r, int $g, int $b): int
    {}

    private function degradeHexColorToAnsi4(int $r, int $g, int $b): int
    {}

    /**
     * Inspired from https://github.com/ajalt/colormath/blob/e464e0da1b014976736cf97250063248fc77b8e7/colormath/src/commonMain/kotlin/com/github/ajalt/colormath/model/Ansi256.kt code (MIT license).
     */
    private function degradeHexColorToAnsi8(int $r, int $g, int $b): int
    {}
}
