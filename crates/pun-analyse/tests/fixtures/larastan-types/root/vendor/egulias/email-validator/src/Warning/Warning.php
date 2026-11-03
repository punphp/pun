<?php

namespace Egulias\EmailValidator\Warning;

abstract class Warning
{
    /**
     * @var int CODE
     */
    public const CODE = 0;

    /**
     * @var string
     */
    protected $message = '';

    /**
     * @var int
     */
    protected $rfcNumber = 0;

    /**
     * @return string
     */
    public function message()
    {}

    /**
     * @return int
     */
    public function code()
    {}

    /**
     * @return int
     */
    public function RFCNumber()
    {}

    /**
     * @return string
     */
    public function __toString(): string
    {}
}
