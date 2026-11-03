<?php

namespace Egulias\EmailValidator\Validation\Extra;

use \Spoofchecker;
use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Result\SpoofEmail;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Validation\EmailValidation;

class SpoofCheckValidation implements EmailValidation
{
    /**
     * @var InvalidEmail|null
     */
    private $error;

    public function __construct()
    {}

    public function isValid(string $email, EmailLexer $emailLexer) : bool
    {}

    public function getError() : ?InvalidEmail
    {}

    public function getWarnings() : array
    {}
}
