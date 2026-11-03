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

use Ramsey\Uuid\Builder\UuidBuilderInterface;
use Ramsey\Uuid\Exception\InvalidArgumentException;
use Ramsey\Uuid\Exception\InvalidUuidStringException;
use Ramsey\Uuid\Uuid;
use Ramsey\Uuid\UuidInterface;

use function bin2hex;
use function hex2bin;
use function implode;
use function sprintf;
use function str_replace;
use function strlen;
use function substr;

/**
 * StringCodec encodes and decodes RFC 9562 (formerly RFC 4122) UUIDs
 *
 * @immutable
 */
class StringCodec implements CodecInterface
{
    /**
     * Constructs a StringCodec
     *
     * @param UuidBuilderInterface $builder The builder to use when encoding UUIDs
     */
    public function __construct(private UuidBuilderInterface $builder)
    {}

    public function encode(UuidInterface $uuid): string
    {}

    /**
     * @return non-empty-string
     */
    public function encodeBinary(UuidInterface $uuid): string
    {}

    /**
     * @throws InvalidUuidStringException
     *
     * @inheritDoc
     */
    public function decode(string $encodedUuid): UuidInterface
    {}

    public function decodeBytes(string $bytes): UuidInterface
    {}

    /**
     * Returns the UUID builder
     */
    protected function getBuilder(): UuidBuilderInterface
    {}

    /**
     * Returns a byte string of the UUID
     */
    protected function getBytes(string $encodedUuid): string
    {}
}
