<?php

namespace Egulias\EmailValidator\Parser;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Result\Result;
use Egulias\EmailValidator\Warning\QuotedPart;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Parser\CommentStrategy\CommentStrategy;
use Egulias\EmailValidator\Result\Reason\UnclosedComment;
use Egulias\EmailValidator\Result\Reason\UnOpenedComment;
use Egulias\EmailValidator\Warning\Comment as WarningComment;

class Comment extends PartParser
{
    /**
     * @var int
     */
    private $openedParenthesis = 0;

    /**
     * @var CommentStrategy
     */
    private $commentStrategy;

    public function __construct(EmailLexer $lexer, CommentStrategy $commentStrategy)
    {}

    public function parse(): Result
    {}


    /**
     * @return void
     */
    private function warnEscaping(): void
    {}

    private function noClosingParenthesis(): bool
    {}
}
