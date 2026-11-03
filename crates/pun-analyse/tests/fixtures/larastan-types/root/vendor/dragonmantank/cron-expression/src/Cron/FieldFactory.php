<?php

declare(strict_types=1);

namespace Cron;

use InvalidArgumentException;

/**
 * CRON field factory implementing a flyweight factory.
 *
 * @see http://en.wikipedia.org/wiki/Cron
 */
class FieldFactory implements FieldFactoryInterface
{
    /**
     * @var array<int, FieldInterface> Cache of instantiated fields
     */
    private $fields = [];

    /**
     * Get an instance of a field object for a cron expression position.
     *
     * @param int $position CRON expression position value to retrieve
     *
     * @throws InvalidArgumentException if a position is not valid
     */
    public function getField(int $position): FieldInterface
    {}

    private function instantiateField(int $position): FieldInterface
    {}
}
