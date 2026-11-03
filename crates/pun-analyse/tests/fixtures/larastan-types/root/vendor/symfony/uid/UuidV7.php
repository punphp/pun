<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Uid;

use Symfony\Component\Uid\Exception\InvalidArgumentException;

/**
 * A v7 UUID is lexicographically sortable and contains a 58-bit timestamp and 64 extra unique bits.
 *
 * Within the same millisecond, the unique bits are incremented by a 24-bit random number.
 * This method provides microsecond precision for the timestamp, and minimizes both the
 * risk of collisions and the consumption of the OS' entropy pool.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
class UuidV7 extends Uuid implements TimeBasedUidInterface
{
    protected const TYPE = 7;

    private static string $time = '';
    private static int $subMs = 0;
    private static array $rand = [];
    private static string $seed;
    private static array $seedParts;
    private static int $seedIndex = 0;

    public function __construct(?string $uuid = null)
    {}

    public function getDateTime(): \DateTimeImmutable
    {}

    public static function generate(?\DateTimeInterface $time = null): string
    {}
}
