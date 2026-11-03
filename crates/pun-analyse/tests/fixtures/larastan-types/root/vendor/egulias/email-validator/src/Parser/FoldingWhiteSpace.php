<?php

namespace Egulias\EmailValidator\Parser;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Warning\CFWSNearAt;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Warning\CFWSWithFWS;
use Egulias\EmailValidator\Result\Reason\CRNoLF;
use Egulias\EmailValidator\Result\Reason\AtextAfterCFWS;
use Egulias\EmailValidator\Result\Reason\CRLFAtTheEnd;
use Egulias\EmailValidator\Result\Reason\CRLFX2;
use Egulias\EmailValidator\Result\Reason\ExpectingCTEXT;
use Egulias\EmailValidator\Result\Result;
use Egulias\EmailValidator\Result\ValidEmail;

class  FoldingWhiteSpace extends PartParser
{
    public const FWS_TYPES = [
        EmailLexer::S_SP,
        EmailLexer::S_HTAB,
        EmailLexer::S_CR,
        EmailLexer::S_LF,
        EmailLexer::CRLF
    ];

    public function parse(): Result
    {}

    protected function checkCRLFInFWS(): Result
    {}

    protected function isFWS(): bool
    {}
}
