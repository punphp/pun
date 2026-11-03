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
 * A ULID is lexicographically sortable and contains a 48-bit timestamp and 80-bit of crypto-random entropy.
 *
 * @see https://github.com/ulid/spec
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
class Ulid extends AbstractUid implements TimeBasedUidInterface
{
    public const FORMAT_BINARY = 1;
    public const FORMAT_BASE_32 = 1 << 1;
    public const FORMAT_BASE_58 = 1 << 2;
    public const FORMAT_RFC_4122 = 1 << 3;
    public const FORMAT_RFC_9562 = self::FORMAT_RFC_4122;
    public const FORMAT_ALL = -1;

    protected const NIL = '00000000000000000000000000';
    protected const MAX = '7ZZZZZZZZZZZZZZZZZZZZZZZZZ';

    private static string $time = '';
    private static array $rand = [];

    public function __construct(?string $ulid = null)
    {}

    /**
     * @param int-mask-of<Ulid::FORMAT_*> $format
     */
    public static function isValid(string $ulid/* , int $format = self::FORMAT_BASE_32 */): bool
    {}

    public static function fromString(string $ulid): static
    {}

    public function toBinary(): string
    {}

    /**
     * Returns the identifier as a base32 case insensitive string.
     *
     * @see https://tools.ietf.org/html/rfc4648#section-6
     *
     * @example 09EJ0S614A9FXVG9C5537Q9ZE1 (len=26)
     */
    public function toBase32(): string
    {}

    public function getDateTime(): \DateTimeImmutable
    {}

    public static function generate(?\DateTimeInterface $time = null): string
    {}

    /**
     * @param int-mask-of<Ulid::FORMAT_*> $format
     *
     * @return string|false The base32 string or false if the format doesn't match the input
     */
    private static function transformToBase32(string $ulid, int $format): string|false
    {}

    private static function binaryToBase32(string $ulid): string
    {}
}
