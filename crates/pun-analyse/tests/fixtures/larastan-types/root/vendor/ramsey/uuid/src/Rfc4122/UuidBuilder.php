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

use Ramsey\Uuid\Builder\UuidBuilderInterface;
use Ramsey\Uuid\Codec\CodecInterface;
use Ramsey\Uuid\Converter\NumberConverterInterface;
use Ramsey\Uuid\Converter\Time\UnixTimeConverter;
use Ramsey\Uuid\Converter\TimeConverterInterface;
use Ramsey\Uuid\Exception\UnableToBuildUuidException;
use Ramsey\Uuid\Exception\UnsupportedOperationException;
use Ramsey\Uuid\Math\BrickMathCalculator;
use Ramsey\Uuid\Rfc4122\UuidInterface as Rfc4122UuidInterface;
use Ramsey\Uuid\Uuid;
use Ramsey\Uuid\UuidInterface;
use Throwable;

/**
 * UuidBuilder builds instances of RFC 9562 (formerly 4122) UUIDs
 *
 * @immutable
 */
class UuidBuilder implements UuidBuilderInterface
{
    private TimeConverterInterface $unixTimeConverter;

    /**
     * Constructs the DefaultUuidBuilder
     *
     * @param NumberConverterInterface $numberConverter The number converter to use when constructing the Uuid
     * @param TimeConverterInterface $timeConverter The time converter to use for converting Gregorian time extracted
     *     from version 1, 2, and 6 UUIDs to Unix timestamps
     * @param TimeConverterInterface | null $unixTimeConverter The time converter to use for converter Unix Epoch time
     *     extracted from version 7 UUIDs to Unix timestamps
     */
    public function __construct(
        private NumberConverterInterface $numberConverter,
        private TimeConverterInterface $timeConverter,
        ?TimeConverterInterface $unixTimeConverter = null,
    ) {}

    /**
     * Builds and returns a Uuid
     *
     * @param CodecInterface $codec The codec to use for building this Uuid instance
     * @param string $bytes The byte string from which to construct a UUID
     *
     * @return Rfc4122UuidInterface UuidBuilder returns instances of Rfc4122UuidInterface
     *
     * @pure
     */
    public function build(CodecInterface $codec, string $bytes): UuidInterface
    {}

    /**
     * Proxy method to allow injecting a mock for testing
     *
     * @pure
     */
    protected function buildFields(string $bytes): FieldsInterface
    {}
}
