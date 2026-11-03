<?php

namespace Egulias\EmailValidator\Parser;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Result\Reason\ConsecutiveDot;
use Egulias\EmailValidator\Result\Result;
use Egulias\EmailValidator\Result\ValidEmail;
use Egulias\EmailValidator\Warning\Warning;

abstract class PartParser
{
    /**
     * @var Warning[]
     */
    protected $warnings = [];

    /**
     * @var EmailLexer
     */
    protected $lexer;

    public function __construct(EmailLexer $lexer)
    {}

    abstract public function parse(): Result;

    /**
     * @return Warning[]
     */
    public function getWarnings()
    {}

    protected function parseFWS(): Result
    {}

    protected function checkConsecutiveDots(): Result
    {}

    protected function escaped(): bool
    {}
}
