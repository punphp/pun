<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Uid;

use Symfony\Component\Uid\Exception\InvalidArgumentException;

/**
 * @author Nicolas Grekas <p@tchwork.com>
 */
abstract class AbstractUid implements \JsonSerializable, \Stringable, HashableInterface
{
    /**
     * The identifier in its canonic representation.
     */
    protected string $uid;

    /**
     * Whether the passed value is valid for the constructor of the current class.
     */
    abstract public static function isValid(string $uid): bool;

    /**
     * Creates an AbstractUid from an identifier represented in any of the supported formats.
     *
     * @throws InvalidArgumentException When the passed value is not valid
     */
    abstract public static function fromString(string $uid): static;

    /**
     * @throws InvalidArgumentException When the passed value is not valid
     */
    public static function fromBinary(string $uid): static
    {}

    /**
     * @throws InvalidArgumentException When the passed value is not valid
     */
    public static function fromBase58(string $uid): static
    {}

    /**
     * @throws InvalidArgumentException When the passed value is not valid
     */
    public static function fromBase32(string $uid): static
    {}

    /**
     * @param string $uid A valid RFC 9562/4122 uid
     *
     * @throws InvalidArgumentException When the passed value is not valid
     */
    public static function fromRfc4122(string $uid): static
    {}

    /**
     * Returns the identifier as a raw binary string.
     *
     * @return non-empty-string
     */
    abstract public function toBinary(): string;

    /**
     * Returns the identifier as a base58 case-sensitive string.
     *
     * @example 2AifFTC3zXgZzK5fPrrprL (len=22)
     *
     * @return non-empty-string
     */
    public function toBase58(): string
    {}

    /**
     * Returns the identifier as a base32 case-insensitive string.
     *
     * @see https://tools.ietf.org/html/rfc4648#section-6
     *
     * @example 09EJ0S614A9FXVG9C5537Q9ZE1 (len=26)
     *
     * @return non-empty-string
     */
    public function toBase32(): string
    {}

    /**
     * Returns the identifier as a RFC 9562/4122 case-insensitive string.
     *
     * @see https://datatracker.ietf.org/doc/html/rfc9562/#section-4
     *
     * @example 09748193-048a-4bfb-b825-8528cf74fdc1 (len=36)
     *
     * @return non-empty-string
     */
    public function toRfc4122(): string
    {}

    /**
     * Returns the identifier as a prefixed hexadecimal case insensitive string.
     *
     * @example 0x09748193048a4bfbb8258528cf74fdc1 (len=34)
     *
     * @return non-empty-string
     */
    public function toHex(): string
    {}

    /**
     * Returns whether the argument is an AbstractUid and contains the same value as the current instance.
     */
    public function equals(mixed $other): bool
    {}

    /**
     * @return non-empty-string
     */
    public function hash(): string
    {}

    public function compare(self $other): int
    {}

    /**
     * @return non-empty-string
     */
    final public function toString(): string
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
}
