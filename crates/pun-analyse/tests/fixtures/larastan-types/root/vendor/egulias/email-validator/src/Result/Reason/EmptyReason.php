<?php

namespace Egulias\EmailValidator\Result\Reason;

class EmptyReason implements Reason
{
    public function code() : int
    {}

    public function description() : string
    {}
}
