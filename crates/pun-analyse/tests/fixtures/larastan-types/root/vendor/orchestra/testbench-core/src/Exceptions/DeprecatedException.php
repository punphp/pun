<?php

namespace Orchestra\Testbench\Exceptions;

use PHPUnit\Util\Filter;

/**
 * @internal
 *
 * @codeCoverageIgnore
 */
class DeprecatedException extends PHPUnitErrorException
{
    /**
     * Convert exception to string.
     *
     * @return string
     */
    public function __toString(): string
    {}
}
