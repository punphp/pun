<?php

namespace Egulias\EmailValidator;

use Egulias\EmailValidator\Result\Result;
use Egulias\EmailValidator\Parser\LocalPart;
use Egulias\EmailValidator\Parser\DomainPart;
use Egulias\EmailValidator\Result\ValidEmail;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Warning\EmailTooLong;
use Egulias\EmailValidator\Result\Reason\NoLocalPart;

class EmailParser extends Parser
{
    public const EMAIL_MAX_LENGTH = 254;

    /**
     * @var string
     */
    protected $domainPart = '';

    /**
     * @var string
     */
    protected $localPart = '';

    public function parse(string $str): Result
    {}

    protected function preLeftParsing(): Result
    {}

    protected function parseLeftFromAt(): Result
    {}

    protected function parseRightFromAt(): Result
    {}

    private function processLocalPart(): Result
    {}

    private function processDomainPart(): Result
    {}

    public function getDomainPart(): string
    {}

    public function getLocalPart(): string
    {}

    private function addLongEmailWarning(string $localPart, string $parsedDomainPart): void
    {}
}
