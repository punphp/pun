<?php

namespace Egulias\EmailValidator\Parser\CommentStrategy;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Result\Result;
use Egulias\EmailValidator\Result\ValidEmail;
use Egulias\EmailValidator\Warning\CFWSNearAt;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Result\Reason\ExpectingATEXT;
use Egulias\EmailValidator\Warning\Warning;

class LocalComment implements CommentStrategy
{
    /**
     * @var array<int, Warning>
     */
    private $warnings = [];

    public function exitCondition(EmailLexer $lexer, int $openedParenthesis): bool
    {}

    public function endOfLoopValidations(EmailLexer $lexer): Result
    {}

    public function getWarnings(): array
    {}
}
