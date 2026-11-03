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

use Ramsey\Uuid\Exception\InvalidArgumentException;
use ValueError;

use function is_numeric;
use function sprintf;
use function str_starts_with;

/**
 * A value object representing a decimal
 *
 * This class exists for type-safety purposes, to ensure that decimals returned from ramsey/uuid methods as strings are
 * truly decimals and not some other kind of string.
 *
 * To support values as true decimals and not as floats or doubles, we store the decimals as strings.
 *
 * @immutable
 */
final class Decimal implements NumberInterface
{
    private string $value;
    private bool $isNegative;

    public function __construct(float | int | string | self $value)
    {}

    public function isNegative(): bool
    {}

    public function toString(): string
    {}

    public function __toString(): string
    {}

    public function jsonSerialize(): string
    {}

    public function serialize(): string
    {}

    /**
     * @return array{string: string}
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
     * @param array{string?: string} $data
     */
    public function __unserialize(array $data): void
    {}
}
