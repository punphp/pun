<?php

namespace Illuminate\Validation\Concerns;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Validation\EmailValidation;

class FilterEmailValidation implements EmailValidation
{
    /**
     * The flags to pass to the filter_var function.
     *
     * @var int|null
     */
    protected $flags;

    /**
     * Create a new validation instance.
     *
     * @param  int|null  $flags
     */
    public function __construct($flags = null)
    {}

    /**
     * Create a new instance which allows any unicode characters in local-part.
     *
     * @return static
     */
    public static function unicode()
    {}

    /**
     * Returns true if the given email is valid.
     *
     * @param  string  $email
     * @param  \Egulias\EmailValidator\EmailLexer  $emailLexer
     * @return bool
     */
    public function isValid(string $email, EmailLexer $emailLexer): bool
    {}

    /**
     * Returns the validation error.
     *
     * @return \Egulias\EmailValidator\Result\InvalidEmail|null
     */
    public function getError(): ?InvalidEmail
    {}

    /**
     * Returns the validation warnings.
     *
     * @return \Egulias\EmailValidator\Warning\Warning[]
     */
    public function getWarnings(): array
    {}
}
