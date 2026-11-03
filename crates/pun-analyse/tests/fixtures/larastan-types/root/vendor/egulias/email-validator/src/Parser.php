<?php

namespace Egulias\EmailValidator;

use Egulias\EmailValidator\Result\Result;
use Egulias\EmailValidator\Result\ValidEmail;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Result\Reason\ExpectingATEXT;

abstract class Parser
{
    /**
     * @var Warning\Warning[]
     */
    protected $warnings = [];

    /**
     * @var EmailLexer
     */
    protected $lexer;

    /**
     * id-left "@" id-right
     */
    abstract protected function parseRightFromAt(): Result;
    abstract protected function parseLeftFromAt(): Result;
    abstract protected function preLeftParsing(): Result;


    public function __construct(EmailLexer $lexer)
    {}

    public function parse(string $str): Result
    {}

    /**
     * @return Warning\Warning[]
     */
    public function getWarnings(): array
    {}

    protected function hasAtToken(): bool
    {}
}
