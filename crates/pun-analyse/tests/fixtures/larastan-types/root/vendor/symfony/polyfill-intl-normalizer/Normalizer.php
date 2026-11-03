<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Intl\Normalizer;

/**
 * Normalizer is a PHP fallback implementation of the Normalizer class provided by the intl extension.
 *
 * It has been validated with Unicode 6.3 Normalization Conformance Test.
 * See http://www.unicode.org/reports/tr15/ for detailed info about Unicode normalizations.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @internal
 */
class Normalizer
{
    public const FORM_D = \Normalizer::FORM_D;
    public const FORM_KD = \Normalizer::FORM_KD;
    public const FORM_C = \Normalizer::FORM_C;
    public const FORM_KC = \Normalizer::FORM_KC;
    public const NFD = \Normalizer::NFD;
    public const NFKD = \Normalizer::NFKD;
    public const NFC = \Normalizer::NFC;
    public const NFKC = \Normalizer::NFKC;

    private static $C;
    private static $D;
    private static $KD;
    private static $cC;
    private static $rawD;
    private static $rawKD;
    private static $ulenMask = ["\xC0" => 2, "\xD0" => 2, "\xE0" => 3, "\xF0" => 4];
    private static $ASCII = "\x20\x65\x69\x61\x73\x6E\x74\x72\x6F\x6C\x75\x64\x5D\x5B\x63\x6D\x70\x27\x0A\x67\x7C\x68\x76\x2E\x66\x62\x2C\x3A\x3D\x2D\x71\x31\x30\x43\x32\x2A\x79\x78\x29\x28\x4C\x39\x41\x53\x2F\x50\x22\x45\x6A\x4D\x49\x6B\x33\x3E\x35\x54\x3C\x44\x34\x7D\x42\x7B\x38\x46\x77\x52\x36\x37\x55\x47\x4E\x3B\x4A\x7A\x56\x23\x48\x4F\x57\x5F\x26\x21\x4B\x3F\x58\x51\x25\x59\x5C\x09\x5A\x2B\x7E\x5E\x24\x40\x60\x7F\x00\x01\x02\x03\x04\x05\x06\x07\x08\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";

    public static function isNormalized(string $string, int $form = self::FORM_C)
    {}

    public static function getRawDecomposition(string $string, int $form = self::FORM_C)
    {}

    public static function normalize(string $string, int $form = self::FORM_C)
    {}

    private static function recompose($s)
    {}

    private static function decompose($s, $c)
    {}

    private static function getData($file)
    {}
}
