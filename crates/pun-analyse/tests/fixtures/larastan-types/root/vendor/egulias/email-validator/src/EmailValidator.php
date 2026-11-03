<?php

namespace Egulias\EmailValidator;

use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Validation\EmailValidation;

class EmailValidator
{
    /**
     * @var EmailLexer
     */
    private $lexer;

    /**
     * @var Warning\Warning[]
     */
    private $warnings = [];

    /**
     * @var ?InvalidEmail
     */
    private $error;

    public function __construct()
    {}

    /**
     * @param string          $email
     * @param EmailValidation $emailValidation
     * @return bool
     */
    public function isValid(string $email, EmailValidation $emailValidation)
    {}

    /**
     * @return boolean
     */
    public function hasWarnings()
    {}

    /**
     * @return array
     */
    public function getWarnings()
    {}

    /**
     * @return InvalidEmail|null
     */
    public function getError()
    {}
}
