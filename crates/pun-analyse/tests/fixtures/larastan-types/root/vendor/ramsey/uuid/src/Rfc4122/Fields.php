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

namespace Ramsey\Uuid\Rfc4122;

use Ramsey\Uuid\Exception\InvalidArgumentException;
use Ramsey\Uuid\Fields\SerializableFieldsTrait;
use Ramsey\Uuid\Type\Hexadecimal;
use Ramsey\Uuid\Uuid;

use function bin2hex;
use function dechex;
use function hexdec;
use function sprintf;
use function str_pad;
use function strlen;
use function substr;
use function unpack;

use const STR_PAD_LEFT;

/**
 * RFC 9562 (formerly RFC 4122) variant UUIDs consist of a set of named fields
 *
 * Internally, this class represents the fields together as a 16-byte binary string.
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
     * @throws InvalidArgumentException if the byte string does not represent an RFC 9562 (formerly RFC 4122) UUID
     * @throws InvalidArgumentException if the byte string does not contain a valid version
     */
    public function __construct(private string $bytes)
    {}

    /**
     * @pure
     */
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

    /**
     * Returns the full 60-bit timestamp, without the version
     *
     * For version 2 UUIDs, the time_low field is the local identifier and should not be returned as part of the time.
     * For this reason, we set the bottom 32 bits of the timestamp to 0's. As a result, there is some loss of timestamp
     * fidelity, for version 2 UUIDs. The timestamp can be off by a range of 0 to 429.4967295 seconds (or 7 minutes, 9
     * seconds, and 496,730 microseconds).
     *
     * For version 6 UUIDs, the timestamp order is reversed from the typical RFC 9562 (formerly RFC 4122) order (the
     * time bits are in the correct bit order, so that it is monotonically increasing). In returning the timestamp
     * value, we put the bits in the order: time_low + time_mid + time_hi.
     */
    public function getTimestamp(): Hexadecimal
    {}

    public function getVersion(): ?int
    {}

    private function isCorrectVariant(): bool
    {}
}
