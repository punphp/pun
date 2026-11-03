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
use Countable;
use DateTimeImmutable;
use DateTimeInterface;
use DateTimeZone;
use Exception;
use League\Uri\HostRecord;
use League\Uri\TypeConverter;
use LogicException;
use TypeError;
use UnitEnum;
use ValueError;

use function array_key_exists;
use function array_map;
use function array_values;
use function count;
use function is_array;
use function is_int;
use function is_string;

/**
 * @implements ArrayAccess<string, null|string|array<string|null>>
 */
final class ExtractionResult implements ArrayAccess, Countable
{
    /**
     * @param array<string, ExtractedValue> $variables
     * @param list<string> $names
     * @param list<string> $missingNames
     * @param list<ExtractionErrorReason> $reasons
     */
    private function __construct(
        private readonly array $variables,
        private readonly array $names,
        private readonly array $missingNames,
        private readonly array $reasons,
    ) {}

    public static function empty(): self
    {}

    public static function failure(VariableCanNotBeExtracted $exception, Template $template): self
    {}

    public static function success(iterable $variables = []): self
    {}

    /**
     * @throws VariableCanNotBeExtracted
     */
    public function reconcile(self $other): self
    {}

    /**
     * Returns true if the extraction is successful.
     */
    public function isSuccessful(): bool
    {}

    /**
     * @return list<ExtractionErrorReason>
     */
    public function reasons(): array
    {}

    /**
     * Returns the list of all variable names.
     *
     * @return list<string>
     */
    public function names(): array
    {}

    /**
     * Returns the list of variable names missing from the extraction.
     *
     * @return list<string>
     */
    public function missingNames(): array
    {}

    /**
     * Tells whether some variables are attached to the result.
     */
    public function hasValue(): bool
    {}

    /**
     * Returns the number of found variables.
     */
    public function count(): int
    {}

    /**
     * @return null|string|array<string|null>
     */
    public function offsetGet(mixed $offset): null|string|array
    {}

    public function offsetExists(mixed $offset): bool
    {}

    public function offsetUnset(mixed $offset): never
    {}

    public function offsetSet(mixed $offset, mixed $value): never
    {}

    /**
     * @return array<string, array<string>|string|null>
     */
    public function variables(): array
    {}

    public function fetch(string|int $variableName): ?ExtractedValue
    {}

    public function array(int|string $name): array
    {}

    public function string(int|string $name, ?string $default = null): ?string
    {}

    public function host(int|string $name, ?string $default = null): ?string
    {}

    public function strings(int|string $name, ?string $default = null): array
    {}

    public function hosts(int|string $name, ?string $default = null): array
    {}

    public function integer(int|string $name, ?int $default = null): ?int
    {}

    /**
     * @return array<int>
     */
    public function integers(int|string $name, ?int $default = null): array
    {}

    public function float(int|string $name, ?float $default = null): ?float
    {}

    /**
     * @return array<float>
     */
    public function floats(int|string $name, ?float $default = null): array
    {}

    public function boolean(int|string $name, ?bool $default = null): ?bool
    {}

    /**
     * @return array<bool>
     */
    public function booleans(int|string $name, ?bool $default = null): array
    {}

    /**
     * @param class-string<UnitEnum> $enumClass
     */
    public function enum(int|string $name, string $enumClass, ?UnitEnum $default = null): ?UnitEnum
    {}

    /**
     * @param class-string<UnitEnum> $enumClass
     *
     * @return array<UnitEnum>
     */
    public function enums(int|string $name, string $enumClass, ?UnitEnum $default = null): array
    {}

    /**
     * @param non-empty-string $format
     *
     * @throws Exception
     */
    public function date(
        int|string $name,
        string $format,
        DateTimeZone|string $timezone = 'UTC',
        ?DateTimeInterface $default = null
    ): ?DateTimeImmutable {}

    /**
     * @param non-empty-string $format
     *
     * @throws Exception
     *
     * @return array<DateTimeImmutable>
     */
    public function dates(int|string $name, string $format, DateTimeZone|string $timezone = 'UTC', ?DateTimeInterface $default = null): array
    {}
}
