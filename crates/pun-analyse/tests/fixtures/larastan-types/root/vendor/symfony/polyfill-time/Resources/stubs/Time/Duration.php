<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Time;

if (\PHP_VERSION_ID < 80600) {
    /**
     * @see https://wiki.php.net/rfc/duration_class
     */
    final class Duration
    {
        private const NANOS_PER_SECOND = 1_000_000_000;

        /**
         * Maximum seconds accepted by Duration.
         *
         * One second is reserved to avoid overflowing when arithmetic operations
         * temporarily require carrying nanoseconds into the seconds component.
         */
        private const MAX_SECONDS = \PHP_INT_SIZE >= 8 ? 9_223_372_035 : \PHP_INT_MAX;

        private const RANGE_ERROR = \PHP_INT_SIZE >= 8
            ? 'The maximum representable range is 9_223_372_035 seconds (roughly 292 years)'
            : 'The maximum representable range is 2_147_483_647 seconds (roughly 68 years)';

        /**
         * Regular expression to parse an ISO-8601 duration string.
         *
         * Year, Month, Week and Day components are intentionally not supported.
         * The biggest supported component is Hour.
         */
        private const REGEXP_ISO8601 = '@^
            PT
            (?=(?:\d+H|\d+M|\d+S)) # look ahead to avoid PT without argument
            (?:(?<hour>\d+)H)?
            (?:(?<minute>\d+)M)?
            (?:(?<second>\d+)S)?
        $@x';

        /**
         * Regular expression matching any ISO-8601 duration, including the
         * date components that Duration does not support.
         */
        private const REGEXP_ISO8601_ANY = '@^
            P
            (?:(?<year>\d+)Y)?
            (?:(?<month>\d+)M)?
            (?:(?<week>\d+)W)?
            (?:(?<day>\d+)D)?
            (?:T(?:\d+H)?(?:\d+M)?(?:\d+S)?)?
        $@x';

        private function __construct(
            public readonly int $seconds,
            public readonly int $nanoseconds,
            public readonly bool $negative,
        ) {}

        /**
         * Create a duration representing $seconds seconds and $nanoseconds nanoseconds. Neither parameter
         * may be negative. $nanoseconds must be less than 1_000_000_000 (the number of nanoseconds in a
         * second).
         *
         * This constructor creates a Duration from its “atomic” components.
         *
         * @throws TimeException
         */
        public static function fromSeconds(int $seconds, int $nanoseconds = 0): self
        {}

        /**
         * Create a duration representing $nanoseconds nanoseconds. $nanoseconds must not be negative.
         *
         * @throws TimeException
         */
        public static function fromNanoseconds(int $nanoseconds): self
        {}

        /**
         * Create a duration representing $microseconds microseconds. $microseconds must not be negative.
         *
         * @throws TimeException
         */
        public static function fromMicroseconds(int $microseconds): self
        {}

        /**
         * Create a duration representing $milliseconds milliseconds. $milliseconds must not be negative.
         *
         * @throws TimeException
         */
        public static function fromMilliseconds(int $milliseconds): self
        {}

        /**
         * Create a duration representing $minutes minutes. $minutes must not be negative.
         *
         * @throws TimeException
         */
        public static function fromMinutes(int $minutes): self
        {}

        /**
         * Create a duration representing $hours hours. $hours must not be negative.
         *
         * @throws TimeException
         */
        public static function fromHours(int $hours): self
        {}

        /**
         * Parse a ISO-8601 period. ISO-8601 periods with a date component will be rejected.
         * The biggest allowed component is H.
         *
         * @throws TimeException
         */
        public static function fromIso8601DurationString(string $specification): self
        {}

        /**
         * Negates the duration.
         *
         * @return self -$this
         */
        public function negate(): self
        {}

        /**
         * Returns the absolute value of the duration.
         *
         * @return self abs($this)
         */
        public function absolute(): self
        {}

        /**
         * Add the given duration to the duration.
         *
         * @return self $this + $duration
         *
         * @throws TimeException
         */
        public function add(self $duration): self
        {}

        /**
         * Subtract the given duration from the duration.
         *
         * @return self $this - $duration
         *
         * @throws TimeException
         */
        public function sub(self $duration): self
        {}

        /**
         * Multiply the length of the duration by the given factor. $factor must not be negative.
         *
         * @return self $this * $factor
         *
         * @throws TimeException
         */
        public function multiplyBy(int $factor): self
        {}

        /**
         * Divide the length of the duration by the given divisor. $divisor must be positive.
         *
         * Fractional nanoseconds will be truncated.
         *
         * @return self $this / $divisor
         *
         * @throws TimeException
         */
        public function divideBy(int $divisor): self
        {}

        /**
         * Returns -1, 0, 1 if $a is less than, equal to, or greater than $b respectively.
         */
        public static function compare(self $a, self $b): int
        {}

        /**
         * The native class is declared readonly, which forbids dynamic properties.
         */
        public function __set(string $name, mixed $value): void
        {}

        /**
         * Adds two non-negative (seconds, nanoseconds) pairs.
         *
         * @return array{0: int, 1: int}
         *
         * @throws TimeException when the sum is out of the representable range
         */
        private static function addMagnitudes(int $seconds, int $nanoseconds, int $addSeconds, int $addNanoseconds): array
        {}

        /**
         * Returns intdiv($remainder * 1_000_000_000 + $nanoseconds, $divisor) for
         * a $remainder that is lower than $divisor.
         *
         * The dividend does not fit in an integer on 32 bit platforms, so the
         * division is done one decimal digit at a time. Every intermediate value
         * stays below 2**53, where floats represent integers exactly.
         */
        private static function divideNanoseconds(int $remainder, int $nanoseconds, int $divisor): int
        {}

        /**
         * @throws TimeException when the duration is out of the representable range
         */
        private static function create(int $seconds, int $nanoseconds, bool $negative): self
        {}

        /**
         * Mirrors the error reporting of timelib_duration_create_from_iso8601string().
         */
        private static function iso8601Error(string $specification): string
        {}
    }
}
