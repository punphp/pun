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

namespace Carbon;

use Closure;
use DateInterval;
use DatePeriod;
use DateTime;
use DateTimeInterface;
use DateTimeZone;
use ReflectionFunction;
use ReflectionNamedType;
use ReflectionType;
use Time\Duration;

final class Callback
{
    private ?ReflectionFunction $function;

    private function __construct(private readonly Closure $closure)
    {}

    public static function fromClosure(Closure $closure): self
    {}

    public static function parameter(mixed $closure, mixed $value, string|int $index = 0): mixed
    {}

    public function getReflectionFunction(): ReflectionFunction
    {}

    public function prepareParameter(mixed $value, string|int $index = 0): mixed
    {}

    public function call(mixed ...$arguments): mixed
    {}

    private function getParameterType(string|int $index): ?ReflectionType
    {}

    /** @return class-string|null */
    private function getPromotedClass(mixed $value): ?string
    {}
}
