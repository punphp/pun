<?php

namespace Egulias\EmailValidator\Validation;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\EmailParser;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Result\Reason\ExceptionFound;
use Egulias\EmailValidator\Warning\Warning;

class RFCValidation implements EmailValidation
{
    /**
     * @var Warning[]
     */
    private array $warnings = [];

    /**
     * @var ?InvalidEmail
     */
    private $error;

    public function isValid(string $email, EmailLexer $emailLexer): bool
    {}

    public function getError(): ?InvalidEmail
    {}

    /**
     * @return Warning[]
     */
    public function getWarnings(): array
    {}
}
