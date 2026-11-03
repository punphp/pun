<?php

namespace Egulias\EmailValidator\Validation;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\MessageIDParser;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Result\Reason\ExceptionFound;
use Egulias\EmailValidator\Warning\Warning;

class MessageIDValidation implements EmailValidation
{

    /**
     * @var Warning[]
     */
    private $warnings = [];

    /**
     * @var ?InvalidEmail
     */
    private $error;

    public function isValid(string $email, EmailLexer $emailLexer): bool
    {}

    /**
     * @return Warning[]
     */
    public function getWarnings(): array
    {}

    public function getError(): ?InvalidEmail
    {}
}
