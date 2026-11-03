<?php

namespace Egulias\EmailValidator\Result\Reason;

/**
 * Used on SERVFAIL, TIMEOUT or other runtime and network errors
 */
class UnableToGetDNSRecord extends NoDNSRecord
{
    public function code() : int
    {}

    public function description() : string
    {}
}
