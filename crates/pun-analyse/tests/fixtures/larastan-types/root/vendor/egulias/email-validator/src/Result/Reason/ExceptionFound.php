<?php

namespace Egulias\EmailValidator\Result\Reason;

class ExceptionFound implements Reason
{
    /**
     * @var \Exception
     */
    private $exception;

    public function __construct(\Exception $exception)
    {}
    public function code() : int
    {}

    public function description() : string
    {}
}
