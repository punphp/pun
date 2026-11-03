<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Helper;

/**
 * Configures the output of the Tree helper.
 *
 * @author Simon André <smn.andre@gmail.com>
 */
final class TreeStyle
{
    public function __construct(
        private readonly string $prefixEndHasNext,
        private readonly string $prefixEndLast,
        private readonly string $prefixLeft,
        private readonly string $prefixMidHasNext,
        private readonly string $prefixMidLast,
        private readonly string $prefixRight,
    ) {}

    public static function box(): self
    {}

    public static function boxDouble(): self
    {}

    public static function compact(): self
    {}

    public static function default(): self
    {}

    public static function light(): self
    {}

    public static function minimal(): self
    {}

    public static function rounded(): self
    {}

    /**
     * @internal
     */
    public function applyPrefixes(\RecursiveTreeIterator $iterator): void
    {}
}
