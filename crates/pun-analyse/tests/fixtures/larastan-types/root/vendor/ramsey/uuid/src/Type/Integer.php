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

use function assert;
use function is_numeric;
use function preg_match;
use function sprintf;
use function substr;

/**
 * A value object representing an integer
 *
 * This class exists for type-safety purposes, to ensure that integers returned from ramsey/uuid methods as strings are
 * truly integers and not some other kind of string.
 *
 * To support large integers beyond PHP_INT_MAX and PHP_INT_MIN on both 64-bit and 32-bit systems, we store the integers
 * as strings.
 *
 * @immutable
 */
final class Integer implements NumberInterface
{
    /**
     * @var numeric-string
     */
    private string $value;

    /**
     * @phpstan-ignore property.readOnlyByPhpDocDefaultValue
     */
    private bool $isNegative = false;

    public function __construct(self | float | int | string $value)
    {}

    public function isNegative(): bool
    {}

    /**
     * @return numeric-string
     *
     * @pure
     */
    public function toString(): string
    {}

    /**
     * @return numeric-string
     */
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

    /**
     * @return numeric-string
     */
    private function prepareValue(float | int | string $value): string
    {}
}
