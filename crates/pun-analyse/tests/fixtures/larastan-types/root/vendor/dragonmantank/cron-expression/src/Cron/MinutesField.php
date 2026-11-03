<?php

declare(strict_types=1);

namespace Cron;

use DateTimeInterface;

/**
 * Minutes field.  Allows: * , / -.
 */
class MinutesField extends AbstractField
{
    /**
     * {@inheritdoc}
     */
    protected $rangeStart = 0;

    /**
     * {@inheritdoc}
     */
    protected $rangeEnd = 59;

    /**
     * {@inheritdoc}
     */
    public function isSatisfiedBy(DateTimeInterface $date, $value, bool $invert):bool
    {}

    /**
     * {@inheritdoc}
     * {@inheritDoc}
     *
     * @param string|null                  $parts
     */
    public function increment(DateTimeInterface &$date, $invert = false, $parts = null): FieldInterface
    {}
}
