<?php

declare(strict_types=1);

/**
 * This file is part of the Carbon package.
 *
 * (c) Brian Nesbitt <brian@nesbot.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Carbon\Traits;

use Carbon\Exceptions\InvalidFormatException;
use Carbon\FactoryImmutable;
use DateTimeZone;
use ReturnTypeWillChange;
use Throwable;

/**
 * Trait Serialization.
 *
 * Serialization and JSON stuff.
 *
 * Depends on the following properties:
 *
 * @property int $year
 * @property int $month
 * @property int $daysInMonth
 * @property int $quarter
 *
 * Depends on the following methods:
 *
 * @method string|static locale(string $locale = null, string ...$fallbackLocales)
 * @method string        toJSON()
 */
trait Serialization
{
    use ObjectInitialisation;

    /**
     * List of key to use for dump/serialization.
     *
     * @var string[]
     */
    protected array $dumpProperties = ['date', 'timezone_type', 'timezone'];

    /**
     * Locale to dump comes here before serialization.
     *
     * @var string|null
     */
    protected $dumpLocale;

    /**
     * Embed date properties to dump in a dedicated variables so it won't overlap native
     * DateTime ones.
     *
     * @var array|null
     */
    protected $dumpDateProperties;

    /**
     * Return a serialized string of the instance.
     */
    public function serialize(): string
    {}

    /**
     * Create an instance from a serialized string.
     *
     * Unserializing object can lead to arbitrary execution of period filters, and assignation
     * of unvalidated values to properties.
     *
     * You should never unserialize a value from not from a trusted source. Ensure you unserialize
     * only strings coming from storage no unauthorized party can read nor write, prefer to exchange
     * data with other systems via static formats such as JSON.
     *
     * If the data has to transit via untrusted systems before unserialization, then you should
     * sign it cryptographically (with HMAC or OpenSSL for example), and verify the signature
     * before unserializing it.
     *
     * If the content of $value is unknown, consider using the allowed_classes option to limit
     * the types of objects that can be built, for instance:
     *
     * @example
     * ```php
     * $object = Carbon::fromSerialized($value, ['allowed_classes' => [Carbon::class, CarbonImmutable::class]]);
     * ```
     *
     * @param \Stringable|string $value
     * @param array              $options example: ['allowed_classes' => [CarbonImmutable::class]]
     *
     * @throws InvalidFormatException
     *
     * @return static
     */
    public static function fromSerialized($value, array $options = []): static
    {}

    /**
     * The __set_state handler.
     *
     * @param string|array $dump
     *
     * @return static
     */
    #[ReturnTypeWillChange]
    public static function __set_state($dump): static
    {}

    /**
     * Returns the values to dump on serialize() called on.
     *
     * @return array
     */
    public function __serialize(): array
    {}

    /**
     * Set locale if specified on unserialize() called.
     */
    public function __unserialize(array $data): void
    {}

    /**
     * Prepare the object for JSON serialization.
     */
    public function jsonSerialize(): mixed
    {}

    /**
     * @deprecated To avoid conflict between different third-party libraries, static setters should not be used.
     *             You should rather transform Carbon object before the serialization.
     *
     * JSON serialize all Carbon instances using the given callback.
     */
    public static function serializeUsing(string|callable|null $format): void
    {}

    /**
     * Cleanup properties attached to the public scope of DateTime when a dump of the date is requested.
     * foreach ($date as $_) {}
     * serializer($date)
     * var_export($date)
     * get_object_vars($date)
     */
    public function cleanupDumpProperties(): self
    {}

    /** @codeCoverageIgnore */
    private function dumpTimezone(mixed $timezone): mixed
    {}
}
