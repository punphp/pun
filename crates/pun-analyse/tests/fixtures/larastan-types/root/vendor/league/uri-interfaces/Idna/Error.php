<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\Uri\Idna;

enum Error: int
{
    case NONE                   = 0;
    case EMPTY_LABEL            = 1;
    case LABEL_TOO_LONG         = 2;
    case DOMAIN_NAME_TOO_LONG   = 4;
    case LEADING_HYPHEN         = 8;
    case TRAILING_HYPHEN        = 0x10;
    case HYPHEN_3_4             = 0x20;
    case LEADING_COMBINING_MARK = 0x40;
    case DISALLOWED             = 0x80;
    case PUNYCODE               = 0x100;
    case LABEL_HAS_DOT          = 0x200;
    case INVALID_ACE_LABEL      = 0x400;
    case BIDI                   = 0x800;
    case CONTEXTJ               = 0x1000;
    case CONTEXTO_PUNCTUATION   = 0x2000;
    case CONTEXTO_DIGITS        = 0x4000;

    public function description(): string
    {}

    public static function filterByErrorBytes(int $errors): array
    {}
}
