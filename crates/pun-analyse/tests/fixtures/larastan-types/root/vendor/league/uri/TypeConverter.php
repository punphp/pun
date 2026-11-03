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

namespace League\Uri;

use BackedEnum;
use DateTimeImmutable;
use DateTimeInterface;
use DateTimeZone;
use Exception;
use ReflectionEnum;
use Stringable;
use UnitEnum;
use ValueError;

use function enum_exists;
use function filter_var;
use function is_bool;
use function is_float;
use function is_int;
use function is_iterable;
use function is_scalar;
use function is_string;
use function trim;

use const FILTER_NULL_ON_FAILURE;
use const FILTER_VALIDATE_BOOLEAN;
use const FILTER_VALIDATE_FLOAT;
use const FILTER_VALIDATE_INT;

final class TypeConverter
{
    private function __construct()
    {}

    public static function toString(mixed $value): ?string
    {}

    public static function toInteger(mixed $value): ?int
    {}

    public static function toFloat(mixed $value): ?float
    {}

    public static function toBoolean(mixed $value): ?bool
    {}

    /**
     * @return array<string>
     */
    public static function toStrings(mixed $values, ?string $default = null): array
    {}

    /**
     * @return array<int>
     */
    public static function toIntegers(mixed $values, ?int $default = null): array
    {}

    /**
     * @return array<float>
     */
    public static function toFloats(mixed $values, ?float $default = null): array
    {}

    /**
     * @return array<bool>
     */
    public static function toBooleans(mixed $values, ?bool $default = null): array
    {}

    /**
     * @param class-string<UnitEnum> $enumClass
     */
    public static function toEnum(mixed $value, string $enumClass): ?UnitEnum
    {}

    /**
     * @template T of UnitEnum
     *
     * @param class-string<T> $enumClass
     * @param ?T $default
    */
    public static function toEnums(mixed $values, string $enumClass, ?UnitEnum $default = null): array
    {}

    /**
     * @template T of BackedEnum
     *
     * @param class-string<T> $enumClass
     * @param array<T> $cases
     *
     * @return ?T
     */
    private static function findBackedEnum(mixed $value, string $enumClass, array $cases): ?UnitEnum
    {}

    /**
     * @template T of UnitEnum
     *
     * @param class-string<T> $enumClass
     * @param array<T> $cases
     *
     * @return ?T
     */
    private static function findEnum(mixed $value, string $enumClass, array $cases): ?UnitEnum
    {}

    /**
     * @param non-empty-string $format
     *
     * @throws Exception if the provided timezone is invalid
     */
    public static function toDateTimeImmutable(mixed $value, string $format, DateTimeZone|string $timezone = 'UTC'): ?DateTimeImmutable
    {}

    /**
     * @param non-empty-string $format
     *
     * @throws Exception
     *
     * @return array<DateTimeImmutable>
     */
    public static function toDateTimeImmutables(
        mixed $values,
        string $format,
        DateTimeZone|string $timezone = 'UTC',
        DateTimeInterface|null $default = null,
    ): array {}

    /**
     * @param non-empty-string $format
     */
    private static function createDateTimeImmutable(mixed $value, string $format, DateTimeZone $timezone): ?DateTimeImmutable
    {}
}
