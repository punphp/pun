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

namespace Ramsey\Uuid\Nonstandard;

use Ramsey\Uuid\Exception\InvalidArgumentException;
use Ramsey\Uuid\Fields\SerializableFieldsTrait;
use Ramsey\Uuid\Rfc4122\FieldsInterface;
use Ramsey\Uuid\Rfc4122\VariantTrait;
use Ramsey\Uuid\Type\Hexadecimal;

use function bin2hex;
use function dechex;
use function hexdec;
use function sprintf;
use function str_pad;
use function strlen;
use function substr;

use const STR_PAD_LEFT;

/**
 * Nonstandard UUID fields do not conform to the RFC 9562 (formerly RFC 4122) standard
 *
 * Since some systems may create nonstandard UUIDs, this implements the {@see FieldsInterface}, so that functionality of
 * a nonstandard UUID is not degraded, in the event these UUIDs are expected to contain RFC 9562 (formerly RFC 4122) fields.
 *
 * Internally, this class represents the fields together as a 16-byte binary string.
 *
 * @immutable
 */
final class Fields implements FieldsInterface
{
    use SerializableFieldsTrait;
    use VariantTrait;

    /**
     * @param string $bytes A 16-byte binary string representation of a UUID
     *
     * @throws InvalidArgumentException if the byte string is not exactly 16 bytes
     */
    public function __construct(private string $bytes)
    {}

    public function getBytes(): string
    {}

    public function getClockSeq(): Hexadecimal
    {}

    public function getClockSeqHiAndReserved(): Hexadecimal
    {}

    public function getClockSeqLow(): Hexadecimal
    {}

    public function getNode(): Hexadecimal
    {}

    public function getTimeHiAndVersion(): Hexadecimal
    {}

    public function getTimeLow(): Hexadecimal
    {}

    public function getTimeMid(): Hexadecimal
    {}

    public function getTimestamp(): Hexadecimal
    {}

    public function getVersion(): ?int
    {}

    public function isNil(): bool
    {}

    public function isMax(): bool
    {}
}
