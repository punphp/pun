<?php

namespace Egulias\EmailValidator\Parser;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Result\Result;
use Egulias\EmailValidator\Result\ValidEmail;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Warning\LocalTooLong;
use Egulias\EmailValidator\Result\Reason\DotAtEnd;
use Egulias\EmailValidator\Result\Reason\DotAtStart;
use Egulias\EmailValidator\Result\Reason\ConsecutiveDot;
use Egulias\EmailValidator\Result\Reason\ExpectingATEXT;
use Egulias\EmailValidator\Parser\CommentStrategy\LocalComment;

class LocalPart extends PartParser
{
    public const INVALID_TOKENS = [
        EmailLexer::S_COMMA => EmailLexer::S_COMMA,
        EmailLexer::S_CLOSEBRACKET => EmailLexer::S_CLOSEBRACKET,
        EmailLexer::S_OPENBRACKET => EmailLexer::S_OPENBRACKET,
        EmailLexer::S_GREATERTHAN => EmailLexer::S_GREATERTHAN,
        EmailLexer::S_LOWERTHAN => EmailLexer::S_LOWERTHAN,
        EmailLexer::S_COLON => EmailLexer::S_COLON,
        EmailLexer::S_SEMICOLON => EmailLexer::S_SEMICOLON,
        EmailLexer::INVALID => EmailLexer::INVALID
    ];

    /**
     * @var string
     */
    private $localPart = '';


    public function parse(): Result
    {}

    protected function validateTokens(bool $hasComments): Result
    {}

    public function localPart(): string
    {}

    private function parseLocalFWS(): Result
    {}

    private function hasDotAtStart(): bool
    {}

    private function parseDoubleQuote(): Result
    {}

    protected function parseComments(): Result
    {}

    private function validateEscaping(): Result
    {}
}
