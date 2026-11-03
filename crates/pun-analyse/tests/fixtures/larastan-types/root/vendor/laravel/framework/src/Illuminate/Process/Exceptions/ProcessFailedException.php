<?php

namespace Illuminate\Process\Exceptions;

use Illuminate\Contracts\Process\ProcessResult;
use RuntimeException;

class ProcessFailedException extends RuntimeException
{
    /**
     * The process result instance.
     *
     * @var \Illuminate\Contracts\Process\ProcessResult
     */
    public $result;

    /**
     * Create a new exception instance.
     *
     * @param  \Illuminate\Contracts\Process\ProcessResult  $result
     */
    public function __construct(ProcessResult $result)
    {}
}
