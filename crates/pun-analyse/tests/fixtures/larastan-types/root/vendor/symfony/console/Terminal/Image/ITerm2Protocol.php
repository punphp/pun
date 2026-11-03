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
 * Handles the iTerm2 Inline Images Protocol (OSC 1337).
 *
 * The iTerm2 protocol uses Operating System Command (OSC) sequences:
 * - Start: ESC ] 1337 ; File= (0x1B 0x5D 0x31 0x33 0x33 0x37 0x3B 0x46 0x69 0x6C 0x65 0x3D)
 * - End: BEL (0x07) or ESC \ (0x1B 0x5C)
 *
 * Format: ESC]1337;File=[arguments]:[base64 data]BEL
 *
 * @see https://iterm2.com/documentation-images.html
 *
 * @author Robin Chalas <robin.chalas@gmail.com>
 *
 * @internal
 */
final class ITerm2Protocol implements ImageProtocolInterface
{
    public const OSC_START = "\x1b]1337;File=";
    public const BEL = "\x07";
    public const ST = "\x1b\\";

    public function detectPastedImage(string $data): bool
    {}

    public function decode(string $data): array
    {}

    public function encode(string $imageData, ?int $maxWidth = null): string
    {}

    public function getName(): string
    {}

    private function detectImageFormat(string $data): ?string
    {}
}
