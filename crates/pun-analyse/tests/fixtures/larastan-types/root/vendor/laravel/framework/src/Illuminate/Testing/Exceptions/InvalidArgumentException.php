<?php

namespace Illuminate\Testing\Exceptions;

use PHPUnit\Framework\Exception;

class InvalidArgumentException extends Exception
{
    /**
     * Creates a new exception for an invalid argument.
     *
     * @param  int  $argument
     * @param  string  $type
     * @return static
     */
    public static function create(int $argument, string $type): static
    {}
}
