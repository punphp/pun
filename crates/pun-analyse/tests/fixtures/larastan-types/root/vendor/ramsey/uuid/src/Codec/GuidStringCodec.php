<?php

/**
 * This file is part of the ramsey/uuid library
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 *
 * @copyright Copyright (c) Ben Ramsey <ben@benramsey.com>
 * @license http://opensource.org/licenses/MIT MIT
 */

declare(strict_types=1);

namespace Ramsey\Uuid\Codec;

use Ramsey\Uuid\Guid\Guid;
use Ramsey\Uuid\UuidInterface;

use function bin2hex;
use function sprintf;
use function substr;

/**
 * GuidStringCodec encodes and decodes globally unique identifiers (GUID)
 *
 * @see Guid
 *
 * @immutable
 */
class GuidStringCodec extends StringCodec
{
    public function encode(UuidInterface $uuid): string
    {}

    public function decode(string $encodedUuid): UuidInterface
    {}

    public function decodeBytes(string $bytes): UuidInterface
    {}

    /**
     * Swaps bytes according to the GUID rules
     */
    private function swapBytes(string $bytes): string
    {}
}
