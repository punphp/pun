<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\Uri\UriTemplate;

use function array_chunk;
use function array_column;
use function array_key_exists;
use function array_map;
use function array_unique;
use function count;
use function explode;
use function in_array;
use function is_array;
use function is_string;

final class ExtractedValue
{
    public readonly bool $isPartial;

    /**
     * @throws VariableCanNotBeExtracted
     */
    private function __construct(
        public readonly array|string|null $value,
        private readonly int $maxLength = -1,
        public readonly array $asList = [],
    ) {}

    public static function fromNull(): self
    {}

    public static function fromString(string $value, int $maxLength = -1): self
    {}

    public static function fromArray(array $value): self
    {}

    /**
     * @throws VariableCanNotBeExtracted
     */
    public static function fromList(string $value, Operator $operator, VarSpecifier $varSpecifier): self
    {}

    /**
     * Extracts an exploded variable from a positional representation.
     *
     * Positional exploded values may contain either plain values or name/value
     * pairs, but not both representations at the same time.
     */
    public static function fromUnnamedList(string $value, Operator $operator): ExtractedValue
    {}

    /**
     * Creates an extracted value from a path parameter containing alternating
     * key/value pairs.
     *
     * The value is split into comma-separated key/value pairs before decoding,
     * so percent-encoded commas (`%2C`) remain part of the corresponding key or value.
     *
     * @param non-empty-string $value
     *
     * @throws VariableCanNotBeExtracted if the value is the empty string
     */
    public static function fromNamedValue(string $value, Operator $operator): self
    {}

    /**
     * Extracts an exploded variable from a positional representation.
     *
     * Positional exploded values may contain either plain values or name/value
     * pairs, but not both representations at the same time.
     */
    private static function fromGenericList(string $value, Operator $operator): ExtractedValue
    {}

    private static function fromPositionalPairs(string $value, Operator $operator): self
    {}

    /**
     * Extracts an exploded variable from a named representation.
     *
     * The value may consist of repeated occurrences of the variable name or of
     * name/value pairs. When all pairs use the variable's name, the values are
     * returned as a list. Otherwise, the complete name/value mapping is returned,
     * provided the variable's name is not mixed with other names.
     */
    public static function fromNamedList(string $value, Operator $operator, VarSpecifier $varSpecifier): self
    {}

    public static function fromValue(string $value, Operator $operator, VarSpecifier $varSpecifier): self
    {}

    public function equals(mixed $value): bool
    {}

    public function hasKey(string|int $name): bool
    {}

    /**
     * @throws VariableCanNotBeExtracted
     */
    public function reconcile(self $other): self
    {}
}
