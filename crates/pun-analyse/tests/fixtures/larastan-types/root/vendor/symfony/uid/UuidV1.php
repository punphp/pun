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

/**
 * A v1 UUID contains a 60-bit timestamp and 62 extra unique bits.
 *
 * @author Grégoire Pineau <lyrixx@lyrixx.info>
 */
class UuidV1 extends Uuid implements TimeBasedUidInterface
{
    protected const TYPE = 1;

    private static string $clockSeq;

    public function __construct(?string $uuid = null)
    {}

    public function getDateTime(): \DateTimeImmutable
    {}

    public function getNode(): string
    {}

    public function toV6(): UuidV6
    {}

    public function toV7(): UuidV7
    {}

    public static function generate(?\DateTimeInterface $time = null, ?Uuid $node = null): string
    {}
}
