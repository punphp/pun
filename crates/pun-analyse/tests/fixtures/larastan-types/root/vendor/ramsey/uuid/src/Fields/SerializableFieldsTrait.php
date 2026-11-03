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

namespace Ramsey\Uuid\Fields;

use ValueError;

use function base64_decode;
use function sprintf;
use function strlen;

/**
 * Provides common serialization functionality to fields
 *
 * @immutable
 */
trait SerializableFieldsTrait
{
    /**
     * @param string $bytes The bytes that comprise the fields
     */
    abstract public function __construct(string $bytes);

    /**
     * Returns the bytes that comprise the fields
     */
    abstract public function getBytes(): string;

    /**
     * Returns a string representation of the object
     */
    public function serialize(): string
    {}

    /**
     * @return array{bytes: string}
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
     * @param array{bytes?: string} $data
     */
    public function __unserialize(array $data): void
    {}
}
