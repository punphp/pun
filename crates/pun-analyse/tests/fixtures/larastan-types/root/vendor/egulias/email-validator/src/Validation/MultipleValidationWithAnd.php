<?php

namespace Egulias\EmailValidator\Validation;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Validation\Exception\EmptyValidationList;
use Egulias\EmailValidator\Result\MultipleErrors;
use Egulias\EmailValidator\Warning\Warning;

class MultipleValidationWithAnd implements EmailValidation
{
    /**
     * If one of validations fails, the remaining validations will be skipped.
     * This means MultipleErrors will only contain a single error, the first found.
     */
    public const STOP_ON_ERROR = 0;

    /**
     * All of validations will be invoked even if one of them got failure.
     * So MultipleErrors will contain all causes.
     */
    public const ALLOW_ALL_ERRORS = 1;

    /**
     * @var Warning[]
     */
    private $warnings = [];

    /**
     * @var MultipleErrors|null
     */
    private $error;

    /**
     * @param EmailValidation[] $validations The validations.
     * @param int               $mode        The validation mode (one of the constants).
     */
    public function __construct(private readonly array $validations, private readonly int $mode = self::ALLOW_ALL_ERRORS)
    {}

    /**
     * {@inheritdoc}
     */
    public function isValid(string $email, EmailLexer $emailLexer): bool
    {}

    private function initErrorStorage(): void
    {}

    private function processError(EmailValidation $validation): void
    {}

    private function shouldStop(bool $result): bool
    {}

    /**
     * Returns the validation errors.
     */
    public function getError(): ?InvalidEmail
    {}

    /**
     * @return Warning[]
     */
    public function getWarnings(): array
    {}
}
