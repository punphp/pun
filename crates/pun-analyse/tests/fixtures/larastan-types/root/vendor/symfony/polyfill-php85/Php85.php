<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Polyfill\Php85;

/**
 * @author Pierre Ambroise <pierre27.ambroise@gmail.com>
 * @author Alexander Schranz <alexander@sulu.io>
 *
 * @internal
 */
final class Php85
{
    public static function get_error_handler(): ?callable
    {}

    public static function get_exception_handler(): ?callable
    {}

    public static function array_first(array $array)
    {}

    public static function array_last(array $array)
    {}

    // Taken from CLDR 44 (ICU 74); the exact set is ICU-version dependent and won't match every version of the native locale_is_right_to_left().
    private const RTL_SCRIPTS = [
        'Adlm' => true, 'Arab' => true, 'Armi' => true, 'Avst' => true,
        'Chrs' => true, 'Cprt' => true, 'Elym' => true, 'Hatr' => true,
        'Hebr' => true, 'Hung' => true, 'Khar' => true, 'Lydi' => true,
        'Mand' => true, 'Mani' => true, 'Mend' => true, 'Merc' => true,
        'Mero' => true, 'Narb' => true, 'Nbat' => true, 'Nkoo' => true,
        'Orkh' => true, 'Ougr' => true, 'Palm' => true, 'Phli' => true,
        'Phlp' => true, 'Phnx' => true, 'Prti' => true, 'Rohg' => true,
        'Samr' => true, 'Sarb' => true, 'Sogd' => true, 'Sogo' => true,
        'Syrc' => true, 'Thaa' => true, 'Yezi' => true,
    ];

    private const LANG_TO_SCRIPT = [
        'aeb' => 'Arab',
        'ar' => 'Arab',
        'arc' => 'Armi',
        'arq' => 'Arab',
        'ary' => 'Arab',
        'arz' => 'Arab',
        'bal' => 'Arab',
        'bgn' => 'Arab',
        'ckb' => 'Arab',
        'dv' => 'Thaa',
        'fa' => 'Arab',
        'glk' => 'Arab',
        'haz' => 'Arab',
        'he' => 'Hebr',
        'iw' => 'Hebr',
        'kas' => 'Arab',
        'ks' => 'Arab',
        'lrc' => 'Arab',
        'mzn' => 'Arab',
        'nqo' => 'Nkoo',
        'pnb' => 'Arab',
        'ps' => 'Arab',
        'rhg' => 'Rohg',
        'sam' => 'Samr',
        'sd' => 'Arab',
        'sdh' => 'Arab',
        'skr' => 'Arab',
        'syr' => 'Syrc',
        'ug' => 'Arab',
        'ur' => 'Arab',
        'yi' => 'Hebr',
    ];

    public static function locale_is_right_to_left(string $locale): bool
    {}

    public static function grapheme_levenshtein(string $s1, string $s2, int $insertion_cost = 1, int $replacement_cost = 1, int $deletion_cost = 1, string $locale = '')
    {}
}
