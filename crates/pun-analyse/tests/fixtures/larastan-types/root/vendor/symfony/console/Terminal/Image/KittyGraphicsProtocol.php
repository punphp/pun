<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Terminal\Image;

/**
 * Handles the Kitty Graphics Protocol for terminal image paste/display.
 *
 * The Kitty Graphics Protocol uses Application Programming Command (APC) sequences:
 * - Start: ESC _ G (0x1B 0x5F 0x47)
 * - End: ESC \ (0x1B 0x5C) - also known as ST (String Terminator)
 *
 * Format: ESC_G<control data>;<payload>ESC\
 *
 * @see https://sw.kovidgoyal.net/kitty/graphics-protocol/
 *
 * @author Robin Chalas <robin.chalas@gmail.com>
 *
 * @internal
 */
final class KittyGraphicsProtocol implements ImageProtocolInterface
{
    public const APC_START = "\x1b_G";
    public const ST = "\x1b\\";

    public function detectPastedImage(string $data): bool
    {}

    public function decode(string $data): array
    {}

    public function encode(string $imageData, ?int $maxWidth = null): string
    {}

    public function getName(): string
    {}

    private function parseFormat(string $controlData): ?string
    {}

    private function parseControlValue(string $controlData, string $key): ?string
    {}

    private function detectImageFormat(string $data): ?string
    {}
}
