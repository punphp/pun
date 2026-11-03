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

use ArrayAccess;
use BackedEnum;
use Closure;
use Countable;
use IteratorAggregate;
use League\Uri\StringCoercionMode;
use Stringable;
use Traversable;

use function array_filter;
use function array_key_exists;
use function array_map;
use function count;
use function is_array;

use const ARRAY_FILTER_USE_BOTH;

/**
 * @internal The class exposes the internal representation of variable bags
 *
 * @phpstan-type InputValue string|bool|int|float|array<string|bool|int|float>
 *
 * @implements ArrayAccess<string, InputValue>
 * @implements IteratorAggregate<string, InputValue>
 */
final class VariableBag implements ArrayAccess, Countable, IteratorAggregate
{
    /**
     * @var array<string,string|array<string>>
     */
    private array $variables = [];

    /**
     * @param iterable<array-key, InputValue>|ExtractionResult $variables
     */
    public function __construct(iterable|ExtractionResult $variables = [])
    {}

    public function count(): int
    {}

    public function getIterator(): Traversable
    { yield; }

    public function offsetExists(mixed $offset): bool
    {}

    public function offsetUnset(mixed $offset): void
    {}

    public function offsetSet(mixed $offset, mixed $value): void
    {}

    /**
     * @return null|string|array<string>
     */
    public function offsetGet(mixed $offset): null|string|array
    {}

    /**
     * Tells whether the bag is empty or not.
     */
    public function isEmpty(): bool
    {}

    /**
     * Tells whether the bag is empty or not.
     */
    public function isNotEmpty(): bool
    {}

    public function equals(mixed $value): bool
    {}

    /**
     * Fetches the variable value if none found returns null.
     *
     * @return null|string|array<string>
     */
    public function fetch(string $name): null|string|array
    {}

    /**
     * @param Stringable|InputValue $value
     */
    public function assign(string $name, BackedEnum|Stringable|string|bool|int|float|array|null $value): void
    {}

    /**
     * @param Stringable|InputValue $value
     *
     * @throws TemplateCanNotBeExpanded if the value contains nested list
     */
    private static function normalizeValue(
        BackedEnum|Stringable|string|bool|int|float|array|null $value,
        string $name,
        bool $isNestedListAllowed
    ): array|string {}

    /**
     * Replaces elements from passed variables into the current instance.
     */
    public function replace(VariableBag $variables): self
    {}

    /**
     * Filters elements using the closure.
     */
    public function filter(Closure $fn): self
    {}
}
