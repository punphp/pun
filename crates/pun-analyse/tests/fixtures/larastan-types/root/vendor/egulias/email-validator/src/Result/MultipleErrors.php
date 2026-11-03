<?php

namespace Egulias\EmailValidator\Result;

use Egulias\EmailValidator\Result\Reason\EmptyReason;
use Egulias\EmailValidator\Result\Reason\Reason;

/**
 * @psalm-suppress PropertyNotSetInConstructor
 */
class MultipleErrors extends InvalidEmail
{
    /**
     * @var Reason[]
     */
    private $reasons = [];

    public function __construct()
    {}

    public function addReason(Reason $reason) : void
    {}

    /**
     * @return Reason[]
     */
    public function getReasons() : array
    {}

    public function reason() : Reason
    {}

    public function description() : string
    {}

    public function code() : int
    {}
}
