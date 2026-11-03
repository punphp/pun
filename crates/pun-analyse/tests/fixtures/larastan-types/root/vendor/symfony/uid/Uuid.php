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
 * @author Grégoire Pineau <lyrixx@lyrixx.info>
 *
 * @see https://datatracker.ietf.org/doc/html/rfc9562/#section-6.6 for details about namespaces
 */
class Uuid extends AbstractUid
{
    public const NAMESPACE_DNS = '6ba7b810-9dad-11d1-80b4-00c04fd430c8';
    public const NAMESPACE_URL = '6ba7b811-9dad-11d1-80b4-00c04fd430c8';
    public const NAMESPACE_OID = '6ba7b812-9dad-11d1-80b4-00c04fd430c8';
    public const NAMESPACE_X500 = '6ba7b814-9dad-11d1-80b4-00c04fd430c8';

    public const FORMAT_BINARY = 1;
    public const FORMAT_BASE_32 = 1 << 1;
    public const FORMAT_BASE_58 = 1 << 2;
    public const FORMAT_RFC_4122 = 1 << 3;
    public const FORMAT_RFC_9562 = self::FORMAT_RFC_4122;
    public const FORMAT_ALL = -1;

    protected const TYPE = 0;
    protected const NIL = '00000000-0000-0000-0000-000000000000';
    protected const MAX = 'ffffffff-ffff-ffff-ffff-ffffffffffff';

    public function __construct(string $uuid, bool $checkVariant = false)
    {}

    public static function fromString(string $uuid): static
    {}

    final public static function v1(): UuidV1
    {}

    final public static function v3(self $namespace, string $name): UuidV3
    {}

    final public static function v4(): UuidV4
    {}

    final public static function v5(self $namespace, string $name): UuidV5
    {}

    final public static function v6(): UuidV6
    {}

    final public static function v7(): UuidV7
    {}

    final public static function v8(string $uuid): UuidV8
    {}

    /**
     * @param int-mask-of<Uuid::FORMAT_*> $format
     */
    public static function isValid(string $uuid, int $format = self::FORMAT_RFC_9562): bool
    {}

    public function toBinary(): string
    {}

    /**
     * Returns the identifier as a RFC 9562/4122 case insensitive string.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc9562/#section-4
     *
     * @example 09748193-048a-4bfb-b825-8528cf74fdc1 (len=36)
     */
    public function toRfc4122(): string
    {}

    private static function format(string $uuid, string $version): string
    {}

    /**
     * Transforms a binary string, a base-32 string or a base-58 string to a RFC9562 string.
     *
     * @param int-mask-of<Uuid::FORMAT_*> $format
     *
     * @return string|false The RFC9562 string or false if the format doesn't match the input
     */
    private static function transformToRfc9562(string $uuid, int $format): string|false
    {}
}
