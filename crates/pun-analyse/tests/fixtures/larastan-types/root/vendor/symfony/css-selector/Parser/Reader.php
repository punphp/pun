<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\CssSelector\Parser;

/**
 * CSS selector reader.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class Reader
{
    private static array $anchoredPatterns = [];

    private int $length;
    private int $position = 0;

    public function __construct(
        private string $source,
    ) {}

    public function isEOF(): bool
    {}

    public function getPosition(): int
    {}

    public function getRemainingLength(): int
    {}

    public function getSubstring(int $length, int $offset = 0): string
    {}

    public function getOffset(string $string): int|false
    {}

    public function findPattern(string $pattern): array|false
    {}

    public function moveForward(int $length): void
    {}

    public function moveToEnd(): void
    {}
}
