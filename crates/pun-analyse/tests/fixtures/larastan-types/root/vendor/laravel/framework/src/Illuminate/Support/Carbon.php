<?php

namespace Illuminate\Support;

use Carbon\Carbon as BaseCarbon;
use Carbon\CarbonImmutable as BaseCarbonImmutable;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Dumpable;
use Ramsey\Uuid\Uuid;
use Symfony\Component\Uid\Ulid;

class Carbon extends BaseCarbon
{
    use Conditionable, Dumpable;

    /**
     * {@inheritdoc}
     */
    public static function setTestNow(mixed $testNow = null): void
    {}

    /**
     * Create a Carbon instance from a given ordered UUID or ULID.
     */
    public static function createFromId(Uuid|Ulid|string $id): static
    {}

    /**
     * Get the current date / time plus a given amount of time.
     */
    public function plus(
        int $years = 0,
        int $months = 0,
        int $weeks = 0,
        int $days = 0,
        int $hours = 0,
        int $minutes = 0,
        int $seconds = 0,
        int $microseconds = 0,
        ?bool $overflow = null
    ): static {}

    /**
     * Get the current date / time minus a given amount of time.
     */
    public function minus(
        int $years = 0,
        int $months = 0,
        int $weeks = 0,
        int $days = 0,
        int $hours = 0,
        int $minutes = 0,
        int $seconds = 0,
        int $microseconds = 0,
        ?bool $overflow = null
    ): static {}
}
