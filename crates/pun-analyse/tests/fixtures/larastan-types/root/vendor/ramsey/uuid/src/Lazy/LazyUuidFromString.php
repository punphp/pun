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

namespace Ramsey\Uuid\Lazy;

use DateTimeInterface;
use Ramsey\Uuid\Converter\NumberConverterInterface;
use Ramsey\Uuid\Exception\UnsupportedOperationException;
use Ramsey\Uuid\Fields\FieldsInterface;
use Ramsey\Uuid\Rfc4122\UuidV1;
use Ramsey\Uuid\Rfc4122\UuidV6;
use Ramsey\Uuid\Type\Hexadecimal;
use Ramsey\Uuid\Type\Integer as IntegerObject;
use Ramsey\Uuid\UuidFactory;
use Ramsey\Uuid\UuidInterface;
use ValueError;

use function assert;
use function bin2hex;
use function hex2bin;
use function sprintf;
use function str_replace;
use function substr;

/**
 * Lazy version of a UUID: its format has not been determined yet, so it is mostly only usable for string/bytes
 * conversion. This object optimizes instantiation, serialization and string conversion time, at the cost of increased
 * overhead for more advanced UUID operations.
 *
 * > [!NOTE]
 * > The {@see FieldsInterface} does not declare methods that deprecated API relies upon: the API has been ported from
 * > the {@see \Ramsey\Uuid\Uuid} definition, and is deprecated anyway.
 *
 * > [!NOTE]
 * > The deprecated API from {@see \Ramsey\Uuid\Uuid} is in use here (on purpose): it will be removed once the
 * > deprecated API is gone from this class too.
 *
 * @internal this type is used internally for performance reasons and is not supposed to be directly referenced in consumer libraries.
 */
final class LazyUuidFromString implements UuidInterface
{
    public const VALID_REGEX = '/\A[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\z/ms';

    private ?UuidInterface $unwrapped = null;

    /**
     * @param non-empty-string $uuid
     */
    public function __construct(private string $uuid)
    {}

    public static function fromBytes(string $bytes): self
    {}

    public function serialize(): string
    {}

    /**
     * @return array{string: non-empty-string}
     */
    public function __serialize(): array
    {}

    /**
     * {@inheritDoc}
     *
     * @param non-empty-string $data
     */
    public function unserialize(string $data): void
    {}

    /**
     * @param array{string?: non-empty-string} $data
     */
    public function __unserialize(array $data): void
    {}

    public function getNumberConverter(): NumberConverterInterface
    {}

    /**
     * @inheritDoc
     */
    public function getFieldsHex(): array
    {}

    public function getClockSeqHiAndReservedHex(): string
    {}

    public function getClockSeqLowHex(): string
    {}

    public function getClockSequenceHex(): string
    {}

    public function getDateTime(): DateTimeInterface
    {}

    public function getLeastSignificantBitsHex(): string
    {}

    public function getMostSignificantBitsHex(): string
    {}

    public function getNodeHex(): string
    {}

    public function getTimeHiAndVersionHex(): string
    {}

    public function getTimeLowHex(): string
    {}

    public function getTimeMidHex(): string
    {}

    public function getTimestampHex(): string
    {}

    public function getUrn(): string
    {}

    public function getVariant(): ?int
    {}

    public function getVersion(): ?int
    {}

    public function compareTo(UuidInterface $other): int
    {}

    public function equals(?object $other): bool
    {}

    public function getBytes(): string
    {}

    public function getFields(): FieldsInterface
    {}

    public function getHex(): Hexadecimal
    {}

    public function getInteger(): IntegerObject
    {}

    public function toString(): string
    {}

    public function __toString(): string
    {}

    public function jsonSerialize(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see FieldsInterface} instance. If it is a
     *     {@see Rfc4122FieldsInterface} instance, you may call {@see Rfc4122FieldsInterface::getClockSeqHiAndReserved()}
     *     and use the arbitrary-precision math library of your choice to convert it to a string integer.
     */
    public function getClockSeqHiAndReserved(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see FieldsInterface} instance. If it is a
     *     {@see Rfc4122FieldsInterface} instance, you may call {@see Rfc4122FieldsInterface::getClockSeqLow()} and use
     *     the arbitrary-precision math library of your choice to convert it to a string integer.
     */
    public function getClockSeqLow(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see FieldsInterface} instance. If it is a
     *     {@see Rfc4122FieldsInterface} instance, you may call {@see Rfc4122FieldsInterface::getClockSeq()} and use the
     *     arbitrary-precision math library of your choice to convert it to a string integer.
     */
    public function getClockSequence(): string
    {}

    /**
     * @deprecated This method will be removed in 5.0.0. There is no direct alternative, but the same information may be
     *     obtained by splitting in half the value returned by {@see UuidInterface::getHex()}.
     */
    public function getLeastSignificantBits(): string
    {}

    /**
     * @deprecated This method will be removed in 5.0.0. There is no direct alternative, but the same information may be
     *     obtained by splitting in half the value returned by {@see UuidInterface::getHex()}.
     */
    public function getMostSignificantBits(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see FieldsInterface} instance. If it is a
     *     {@see Rfc4122FieldsInterface} instance, you may call {@see Rfc4122FieldsInterface::getNode()} and use the
     *     arbitrary-precision math library of your choice to convert it to a string integer.
     */
    public function getNode(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see FieldsInterface} instance. If it is a
     *     {@see Rfc4122FieldsInterface} instance, you may call {@see Rfc4122FieldsInterface::getTimeHiAndVersion()} and
     *     use the arbitrary-precision math library of your choice to convert it to a string integer.
     */
    public function getTimeHiAndVersion(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see FieldsInterface} instance. If it is a
     *     {@see Rfc4122FieldsInterface} instance, you may call {@see Rfc4122FieldsInterface::getTimeLow()} and use the
     *     arbitrary-precision math library of your choice to convert it to a string integer.
     */
    public function getTimeLow(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see FieldsInterface} instance. If it is a
     *     {@see Rfc4122FieldsInterface} instance, you may call {@see Rfc4122FieldsInterface::getTimeMid()} and use the
     *     arbitrary-precision math library of your choice to convert it to a string integer.
     */
    public function getTimeMid(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see FieldsInterface} instance. If it is a
     *     {@see Rfc4122FieldsInterface} instance, you may call {@see Rfc4122FieldsInterface::getTimestamp()} and use
     *     the arbitrary-precision math library of your choice to convert it to a string integer.
     */
    public function getTimestamp(): string
    {}

    public function toUuidV1(): UuidV1
    {}

    public function toUuidV6(): UuidV6
    {}

    private function unwrap(): UuidInterface
    {}
}
