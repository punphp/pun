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

namespace Ramsey\Uuid\Guid;

use Ramsey\Uuid\Exception\InvalidArgumentException;
use Ramsey\Uuid\Fields\SerializableFieldsTrait;
use Ramsey\Uuid\Rfc4122\FieldsInterface;
use Ramsey\Uuid\Rfc4122\MaxTrait;
use Ramsey\Uuid\Rfc4122\NilTrait;
use Ramsey\Uuid\Rfc4122\VariantTrait;
use Ramsey\Uuid\Rfc4122\VersionTrait;
use Ramsey\Uuid\Type\Hexadecimal;
use Ramsey\Uuid\Uuid;

use function bin2hex;
use function dechex;
use function hexdec;
use function pack;
use function sprintf;
use function str_pad;
use function strlen;
use function substr;
use function unpack;

use const STR_PAD_LEFT;

/**
 * GUIDs consist of a set of named fields, according to RFC 9562 (formerly RFC 4122)
 *
 * @see Guid
 *
 * @immutable
 */
final class Fields implements FieldsInterface
{
    use MaxTrait;
    use NilTrait;
    use SerializableFieldsTrait;
    use VariantTrait;
    use VersionTrait;

    /**
     * @param string $bytes A 16-byte binary string representation of a UUID
     *
     * @throws InvalidArgumentException if the byte string is not exactly 16 bytes
     * @throws InvalidArgumentException if the byte string does not represent a GUID
     * @throws InvalidArgumentException if the byte string does not contain a valid version
     */
    public function __construct(private string $bytes)
    {}

    public function getBytes(): string
    {}

    public function getTimeLow(): Hexadecimal
    {}

    public function getTimeMid(): Hexadecimal
    {}

    public function getTimeHiAndVersion(): Hexadecimal
    {}

    public function getTimestamp(): Hexadecimal
    {}

    public function getClockSeq(): Hexadecimal
    {}

    public function getClockSeqHiAndReserved(): Hexadecimal
    {}

    public function getClockSeqLow(): Hexadecimal
    {}

    public function getNode(): Hexadecimal
    {}

    public function getVersion(): ?int
    {}

    private function isCorrectVariant(): bool
    {}
}
