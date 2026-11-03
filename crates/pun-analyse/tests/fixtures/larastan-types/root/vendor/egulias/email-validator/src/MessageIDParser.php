<?php

namespace Egulias\EmailValidator;

use Egulias\EmailValidator\Result\Result;
use Egulias\EmailValidator\Parser\IDLeftPart;
use Egulias\EmailValidator\Parser\IDRightPart;
use Egulias\EmailValidator\Result\ValidEmail;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Warning\EmailTooLong;
use Egulias\EmailValidator\Result\Reason\NoLocalPart;

class MessageIDParser extends Parser
{

    public const EMAILID_MAX_LENGTH = 254;

    /**
     * @var string
     */
    protected $idLeft = '';

    /**
     * @var string
     */
    protected $idRight = '';

    public function parse(string $str): Result
    {}

    protected function preLeftParsing(): Result
    {}

    protected function parseLeftFromAt(): Result
    {}

    protected function parseRightFromAt(): Result
    {}

    private function processIDLeft(): Result
    {}

    private function processIDRight(): Result
    {}

    public function getLeftPart(): string
    {}

    public function getRightPart(): string
    {}

    private function addLongEmailWarning(string $localPart, string $parsedDomainPart): void
    {}
}
