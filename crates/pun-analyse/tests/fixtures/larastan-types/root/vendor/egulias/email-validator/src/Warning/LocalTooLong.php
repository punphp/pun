<?php

namespace Egulias\EmailValidator\Warning;

class LocalTooLong extends Warning
{
    public const CODE = 64;
    public const LOCAL_PART_LENGTH = 64;

    public function __construct()
    {}
}
