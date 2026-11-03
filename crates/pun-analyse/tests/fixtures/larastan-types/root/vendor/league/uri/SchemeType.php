<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\Uri;

enum SchemeType
{
    case Opaque;
    case Hierarchical;
    case Unknown;

    public function isOpaque(): bool
    {}

    public function isHierarchical(): bool
    {}

    public function isUnknown(): bool
    {}
}
