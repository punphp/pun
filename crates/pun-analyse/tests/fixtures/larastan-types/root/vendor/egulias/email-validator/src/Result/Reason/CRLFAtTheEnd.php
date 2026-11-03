<?php

namespace Egulias\EmailValidator\Result\Reason;

class CRLFAtTheEnd implements Reason
{
    public const CODE = 149;
    public const REASON = "CRLF at the end";

    public function code() : int
    {}

    public function description() : string
    {}
}
