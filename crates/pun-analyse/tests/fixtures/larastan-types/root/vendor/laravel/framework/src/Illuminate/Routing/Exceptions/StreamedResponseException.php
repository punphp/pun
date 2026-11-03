<?php

namespace Illuminate\Routing\Exceptions;

use Illuminate\Http\Response;
use RuntimeException;
use Throwable;

class StreamedResponseException extends RuntimeException
{
    /**
     * The actual exception thrown during the stream.
     *
     * @var \Throwable
     */
    public $originalException;

    /**
     * Create a new exception instance.
     *
     * @param  \Throwable  $originalException
     */
    public function __construct(Throwable $originalException)
    {}

    /**
     * Render the exception.
     *
     * @return \Illuminate\Http\Response
     */
    public function render()
    {}

    /**
     * Get the actual exception thrown during the stream.
     *
     * @return \Throwable
     */
    public function getInnerException()
    {}
}
