<?php

namespace Illuminate\Routing\Exceptions;

use RuntimeException;

class BackedEnumCaseNotFoundException extends RuntimeException
{
    /**
     * Create a new exception instance.
     *
     * @param  string  $backedEnumClass
     * @param  string  $case
     */
    public function __construct($backedEnumClass, $case)
    {}
}
