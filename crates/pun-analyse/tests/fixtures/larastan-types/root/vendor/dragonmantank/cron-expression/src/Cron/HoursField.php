<?php

declare(strict_types=1);

namespace Cron;

use DateTimeInterface;
use DateTimeZone;

/**
 * Hours field.  Allows: * , / -.
 */
class HoursField extends AbstractField
{
    /**
     * {@inheritdoc}
     */
    protected $rangeStart = 0;

    /**
     * {@inheritdoc}
     */
    protected $rangeEnd = 23;

    /**
     * @var list<array<string, bool|int|string>>|null Transitions returned by DateTimeZone::getTransitions()
     */
    protected $transitions = [];

    /**
     * @var int|null Timestamp of the start of the transitions range
     */
    protected $transitionsStart = null;

    /**
     * @var int|null Timestamp of the end of the transitions range
     */
    protected $transitionsEnd = null;

    /**
     * {@inheritdoc}
     */
    public function isSatisfiedBy(DateTimeInterface $date, $value, bool $invert): bool
    {}

    /**
     * @return non-empty-array<string, bool|int|string>|null
     */
    public function getPastTransition(DateTimeInterface $date): ?array
    {}

    /**
     * {@inheritdoc}
     *
     * @param string|null                  $parts
     */
    public function increment(DateTimeInterface &$date, $invert = false, $parts = null): FieldInterface
    {}
}
