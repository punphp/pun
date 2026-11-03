<?php

namespace Egulias\EmailValidator\Warning;

class NoDNSMXRecord extends Warning
{
    public const CODE = 6;

    public function __construct()
    {}
}
