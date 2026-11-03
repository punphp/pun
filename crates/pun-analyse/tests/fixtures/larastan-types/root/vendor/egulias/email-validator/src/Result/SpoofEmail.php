<?php
namespace Egulias\EmailValidator\Result;

use Egulias\EmailValidator\Result\Reason\SpoofEmail as ReasonSpoofEmail;

class SpoofEmail extends InvalidEmail
{
    public function __construct()
    {}
}
