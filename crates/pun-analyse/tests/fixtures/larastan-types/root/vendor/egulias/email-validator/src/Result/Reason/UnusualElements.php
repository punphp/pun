<?php

namespace Egulias\EmailValidator\Result\Reason;

class UnusualElements implements Reason
{
    /**
     * @var string $element
     */
    private $element;

    public function __construct(string $element)
    {}

    public function code() : int
    {}

    public function description() : string
    {}
}
