<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Uuid;

/**
 * @internal
 *
 * @author Grégoire Pineau <lyrixx@lyrixx.info>
 */
final class Uuid
{
    public const UUID_VARIANT_NCS = 0;
    public const UUID_VARIANT_DCE = 1;
    public const UUID_VARIANT_MICROSOFT = 2;
    public const UUID_VARIANT_OTHER = 3;
    public const UUID_TYPE_DEFAULT = 0;
    public const UUID_TYPE_TIME = 1;
    public const UUID_TYPE_SECURITY = 2;
    public const UUID_TYPE_TIME_V6 = 6;
    public const UUID_TYPE_TIME_V7 = 7;
    public const UUID_TYPE_VENDOR = 8;
    public const UUID_TYPE_MD5 = 3;
    public const UUID_TYPE_DCE = 4; // Deprecated alias
    public const UUID_TYPE_NAME = 1; // Deprecated alias
    public const UUID_TYPE_RANDOM = 4;
    public const UUID_TYPE_SHA1 = 5;
    public const UUID_TYPE_NULL = -1;
    public const UUID_TYPE_INVALID = -42;

    // https://tools.ietf.org/html/rfc4122#section-4.1.4
    // 0x01b21dd213814000 is the number of 100-ns intervals between the
    // UUID epoch 1582-10-15 00:00:00 and the Unix epoch 1970-01-01 00:00:00.
    public const TIME_OFFSET_INT = 0x01B21DD213814000;
    public const TIME_OFFSET_BIN = "\x01\xb2\x1d\xd2\x13\x81\x40\x00";
    public const TIME_OFFSET_COM = "\xfe\x4d\xe2\x2d\xec\x7e\xc0\x00";

    public static function uuid_create($uuid_type = \UUID_TYPE_DEFAULT)
    {}

    public static function uuid_generate_md5($uuid_ns, $name)
    {}

    public static function uuid_generate_sha1($uuid_ns, $name)
    {}

    public static function uuid_is_valid($uuid)
    {}

    public static function uuid_compare($uuid1, $uuid2)
    {}

    public static function uuid_is_null($uuid)
    {}

    public static function uuid_type($uuid)
    {}

    public static function uuid_variant($uuid)
    {}

    public static function uuid_time($uuid)
    {}

    public static function uuid_mac($uuid)
    {}

    public static function uuid_parse($uuid)
    {}

    public static function uuid_unparse($bytes)
    {}

    private static function uuid_generate_random()
    {}

    /**
     * @see http://tools.ietf.org/html/rfc4122#section-4.2.2
     */
    private static function uuid_generate_time($version = 1)
    {}

    /**
     * @see https://www.rfc-editor.org/rfc/rfc9562#section-5.7
     */
    private static function uuid_generate_time_v7()
    {}

    private static function isValid($uuid)
    {}

    private static function parse($uuid)
    {}

    private static function toString($v)
    {}

    private static function toBinary($digits)
    {}

    private static function toDecimal($bytes)
    {}

    private static function binaryAdd($a, $b)
    {}
}
