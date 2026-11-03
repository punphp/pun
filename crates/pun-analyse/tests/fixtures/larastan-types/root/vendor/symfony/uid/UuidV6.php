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
 * A v6 UUID is lexicographically sortable and contains a 60-bit timestamp and 62 extra unique bits.
 *
 * Unlike UUIDv1, this implementation of UUIDv6 doesn't leak the MAC address of the host.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
class UuidV6 extends Uuid implements TimeBasedUidInterface
{
    protected const TYPE = 6;

    private static string $node;

    public function __construct(?string $uuid = null)
    {}

    public function getDateTime(): \DateTimeImmutable
    {}

    public function getNode(): string
    {}

    public function toV7(): UuidV7
    {}

    public static function generate(?\DateTimeInterface $time = null, ?Uuid $node = null): string
    {}
}
