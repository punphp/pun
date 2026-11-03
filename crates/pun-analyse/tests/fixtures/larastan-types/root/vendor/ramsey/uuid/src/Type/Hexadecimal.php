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

use function preg_match;
use function sprintf;
use function substr;

/**
 * A value object representing a hexadecimal number
 *
 * This class exists for type-safety purposes, to ensure that hexadecimal numbers returned from ramsey/uuid methods as
 * strings are truly hexadecimal and not some other kind of string.
 *
 * @immutable
 */
final class Hexadecimal implements TypeInterface
{
    /**
     * @var non-empty-string
     */
    private string $value;

    /**
     * @param self | string $value The hexadecimal value to store
     */
    public function __construct(self | string $value)
    {}

    /**
     * @return non-empty-string
     *
     * @pure
     */
    public function toString(): string
    {}

    /**
     * @return non-empty-string
     */
    public function __toString(): string
    {}

    /**
     * @return non-empty-string
     */
    public function jsonSerialize(): string
    {}

    /**
     * @return non-empty-string
     */
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
     * @return non-empty-string
     */
    private function prepareValue(string $value): string
    {}
}
