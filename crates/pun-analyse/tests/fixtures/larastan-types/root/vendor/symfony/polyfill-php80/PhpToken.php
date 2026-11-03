<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Php80;

/**
 * @author Fedonyuk Anton <info@ensostudio.ru>
 *
 * @internal
 */
class PhpToken implements \Stringable
{
    /**
     * @var int
     */
    public $id;

    /**
     * @var string
     */
    public $text;

    /**
     * @var -1|positive-int
     */
    public $line;

    /**
     * @var int
     */
    public $pos;

    /**
     * @param -1|positive-int $line
     */
    public function __construct(int $id, string $text, int $line = -1, int $position = -1)
    {}

    public function getTokenName(): ?string
    {}

    /**
     * @param int|string|array $kind
     */
    public function is($kind): bool
    {}

    public function isIgnorable(): bool
    {}

    public function __toString(): string
    {}

    /**
     * @return list<static>
     */
    public static function tokenize(string $code, int $flags = 0): array
    {}
}
