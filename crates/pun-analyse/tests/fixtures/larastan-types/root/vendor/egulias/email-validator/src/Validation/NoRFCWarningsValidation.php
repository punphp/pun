<?php

namespace Egulias\EmailValidator\Validation;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Result\Reason\RFCWarnings;

class NoRFCWarningsValidation extends RFCValidation
{
    /**
     * @var InvalidEmail|null
     */
    private $error;

    /**
     * {@inheritdoc}
     */
    public function isValid(string $email, EmailLexer $emailLexer) : bool
    {}

    /**
     * {@inheritdoc}
     */
    public function getError() : ?InvalidEmail
    {}
}
