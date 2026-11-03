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

namespace Ramsey\Uuid;

use DateTimeImmutable;
use DateTimeInterface;
use Ramsey\Uuid\Converter\NumberConverterInterface;
use Ramsey\Uuid\Exception\DateTimeException;
use Ramsey\Uuid\Exception\UnsupportedOperationException;
use Throwable;

use function str_pad;
use function substr;

use const STR_PAD_LEFT;

/**
 * This trait encapsulates deprecated methods for ramsey/uuid; this trait and its methods will be removed in ramsey/uuid 5.0.0.
 *
 * @deprecated This trait and its methods will be removed in ramsey/uuid 5.0.0.
 *
 * @immutable
 */
trait DeprecatedUuidMethodsTrait
{
    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getClockSeqHiAndReserved()} and use the arbitrary-precision math
     *     library of your choice to convert it to a string integer.
     */
    public function getClockSeqHiAndReserved(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getClockSeqHiAndReserved()}.
     */
    public function getClockSeqHiAndReservedHex(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getClockSeqLow()} and use the arbitrary-precision math library of
     *     your choice to convert it to a string integer.
     */
    public function getClockSeqLow(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getClockSeqLow()}.
     */
    public function getClockSeqLowHex(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getClockSeq()} and use the arbitrary-precision math library of
     *     your choice to convert it to a string integer.
     */
    public function getClockSequence(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getClockSeq()}.
     */
    public function getClockSequenceHex(): string
    {}

    /**
     * @deprecated This method will be removed in 5.0.0. There is no alternative recommendation, so plan accordingly.
     */
    public function getNumberConverter(): NumberConverterInterface
    {}

    /**
     * @deprecated In ramsey/uuid version 5.0.0, this will be removed. It is available at {@see UuidV1::getDateTime()}.
     *
     * @return DateTimeImmutable An immutable instance of DateTimeInterface
     *
     * @throws UnsupportedOperationException if UUID is not time-based
     * @throws DateTimeException if DateTime throws an exception/error
     */
    public function getDateTime(): DateTimeInterface
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *
     * @return string[]
     */
    public function getFieldsHex(): array
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
    public function getLeastSignificantBitsHex(): string
    {}

    /**
     * @deprecated This method will be removed in 5.0.0. There is no direct alternative, but the same information may be
     *     obtained by splitting in half the value returned by {@see UuidInterface::getHex()}.
     */
    public function getMostSignificantBits(): string
    {}

    /**
     * @deprecated This method will be removed in 5.0.0. There is no direct alternative, but the same information may be
     *     obtained by splitting in half the value returned by {@see UuidInterface::getHex()}.
     */
    public function getMostSignificantBitsHex(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getNode()} and use the arbitrary-precision math library of your
     *     choice to convert it to a string integer.
     */
    public function getNode(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getNode()}.
     */
    public function getNodeHex(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getTimeHiAndVersion()} and use the arbitrary-precision math
     *     library of your choice to convert it to a string integer.
     */
    public function getTimeHiAndVersion(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getTimeHiAndVersion()}.
     */
    public function getTimeHiAndVersionHex(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getTimeLow()} and use the arbitrary-precision math library of
     *     your choice to convert it to a string integer.
     */
    public function getTimeLow(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getTimeLow()}.
     */
    public function getTimeLowHex(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getTimeMid()} and use the arbitrary-precision math library of
     *     your choice to convert it to a string integer.
     */
    public function getTimeMid(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getTimeMid()}.
     */
    public function getTimeMidHex(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getTimestamp()} and use the arbitrary-precision math library of
     *     your choice to convert it to a string integer.
     */
    public function getTimestamp(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getTimestamp()}.
     */
    public function getTimestampHex(): string
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getVariant()}.
     */
    public function getVariant(): ?int
    {}

    /**
     * @deprecated Use {@see UuidInterface::getFields()} to get a {@see \Ramsey\Uuid\Fields\FieldsInterface} instance.
     *     If it is a {@see \Ramsey\Uuid\Rfc4122\FieldsInterface} instance, you may call
     *     {@see \Ramsey\Uuid\Rfc4122\FieldsInterface::getVersion()}.
     */
    public function getVersion(): ?int
    {}
}
