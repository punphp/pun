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

namespace Ramsey\Uuid\Type;

use Ramsey\Uuid\Exception\UnsupportedOperationException;
use Ramsey\Uuid\Type\Integer as IntegerObject;
use ValueError;

use function json_decode;
use function json_encode;
use function sprintf;

/**
 * A value object representing a timestamp
 *
 * This class exists for type-safety purposes, to ensure that timestamps used by ramsey/uuid are truly timestamp
 * integers and not some other kind of string or integer.
 *
 * @immutable
 */
final class Time implements TypeInterface
{
    private IntegerObject $seconds;
    private IntegerObject $microseconds;

    public function __construct(
        IntegerObject | float | int | string $seconds,
        IntegerObject | float | int | string $microseconds = 0,
    ) {}

    /**
     * @pure
     */
    public function getSeconds(): IntegerObject
    {}

    /**
     * @pure
     */
    public function getMicroseconds(): IntegerObject
    {}

    public function toString(): string
    {}

    public function __toString(): string
    {}

    /**
     * @return string[]
     */
    public function jsonSerialize(): array
    {}

    public function serialize(): string
    {}

    /**
     * @return array{seconds: string, microseconds: string}
     */
    public function __serialize(): array
    {}

    /**
     * Constructs the object from a serialized string representation
     *
     * @param string $data The serialized string representation of the object
     */
    public function unserialize(string $data): void
    {}

    /**
     * @param array{seconds?: string, microseconds?: string} $data
     */
    public function __unserialize(array $data): void
    {}
}
