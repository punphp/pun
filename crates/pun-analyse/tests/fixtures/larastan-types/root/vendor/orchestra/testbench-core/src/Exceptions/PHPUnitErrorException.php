<?php

namespace Orchestra\Testbench\Exceptions;

class PHPUnitErrorException extends \PHPUnit\Framework\Exception
{
    /** {@inheritDoc} */
    public function __construct(string $message, int $code, string $file, int $line, ?\Exception $previous = null)
    {}

    /**
     * Get serializable trace for PHPUnit.
     *
     * @return array
     *
     * @codeCoverageIgnore
     */
    public function getPHPUnitExceptionTrace(): array
    {}
}
