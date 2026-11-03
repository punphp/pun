<?php

namespace Egulias\EmailValidator\Parser;

use Doctrine\Common\Lexer\Token;
use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Warning\TLD;
use Egulias\EmailValidator\Result\Result;
use Egulias\EmailValidator\Result\ValidEmail;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Result\Reason\DotAtEnd;
use Egulias\EmailValidator\Result\Reason\DotAtStart;
use Egulias\EmailValidator\Warning\DeprecatedComment;
use Egulias\EmailValidator\Result\Reason\CRLFAtTheEnd;
use Egulias\EmailValidator\Result\Reason\LabelTooLong;
use Egulias\EmailValidator\Result\Reason\NoDomainPart;
use Egulias\EmailValidator\Result\Reason\ConsecutiveAt;
use Egulias\EmailValidator\Result\Reason\DomainTooLong;
use Egulias\EmailValidator\Result\Reason\CharNotAllowed;
use Egulias\EmailValidator\Result\Reason\DomainHyphened;
use Egulias\EmailValidator\Result\Reason\ExpectingATEXT;
use Egulias\EmailValidator\Parser\CommentStrategy\DomainComment;
use Egulias\EmailValidator\Result\Reason\ExpectingDomainLiteralClose;
use Egulias\EmailValidator\Parser\DomainLiteral as DomainLiteralParser;

class DomainPart extends PartParser
{
    public const DOMAIN_MAX_LENGTH = 253;
    public const LABEL_MAX_LENGTH = 63;

    /**
     * @var string
     */
    protected $domainPart = '';

    /**
     * @var string
     */
    protected $label = '';

    public function parse(): Result
    {}

    private function checkEndOfDomain(): Result
    {}

    private function performDomainStartChecks(): Result
    {}

    private function checkEmptyDomain(): Result
    {}

    private function checkInvalidTokensAfterAT(): Result
    {}

    protected function parseComments(): Result
    {}

    protected function doParseDomainPart(): Result
    {}

     /**
     * @param Token<int, string> $token
     *
     * @return Result
     */
    private function checkNotAllowedChars(Token $token): Result
    {}

    /**
     * @return Result
     */
    protected function parseDomainLiteral(): Result
    {}

    /**
     * @param Token<int, string> $prev
     * @param bool $hasComments
     *
     * @return Result
     */
    protected function checkDomainPartExceptions(Token $prev, bool $hasComments): Result
    {}

    protected function validateTokens(bool $hasComments): Result
    {}

    private function checkLabelLength(bool $isEndOfDomain = false): Result
    {}


    private function isLabelTooLong(string $label): bool
    {}

    private function addTLDWarnings(bool $isTLDMissing): void
    {}

    public function domainPart(): string
    {}
}
