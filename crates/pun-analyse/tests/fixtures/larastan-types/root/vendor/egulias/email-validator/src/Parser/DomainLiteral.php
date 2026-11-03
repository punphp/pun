<?php

namespace Egulias\EmailValidator\Parser;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Result\Result;
use Egulias\EmailValidator\Result\ValidEmail;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Warning\CFWSWithFWS;
use Egulias\EmailValidator\Warning\IPV6BadChar;
use Egulias\EmailValidator\Result\Reason\CRNoLF;
use Egulias\EmailValidator\Warning\IPV6ColonEnd;
use Egulias\EmailValidator\Warning\IPV6MaxGroups;
use Egulias\EmailValidator\Warning\ObsoleteDTEXT;
use Egulias\EmailValidator\Warning\AddressLiteral;
use Egulias\EmailValidator\Warning\IPV6ColonStart;
use Egulias\EmailValidator\Warning\IPV6Deprecated;
use Egulias\EmailValidator\Warning\IPV6GroupCount;
use Egulias\EmailValidator\Warning\IPV6DoubleColon;
use Egulias\EmailValidator\Result\Reason\ExpectingDTEXT;
use Egulias\EmailValidator\Result\Reason\UnusualElements;
use Egulias\EmailValidator\Warning\DomainLiteral as WarningDomainLiteral;

class DomainLiteral extends PartParser
{
    public const IPV4_REGEX = '/\\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/';

    public const OBSOLETE_WARNINGS = [
        EmailLexer::INVALID,
        EmailLexer::C_DEL,
        EmailLexer::S_LF,
        EmailLexer::S_BACKSLASH
    ];

    public function parse(): Result
    {}

    /**
     * @param string $addressLiteral
     * @param int $maxGroups
     */
    public function checkIPV6Tag($addressLiteral, $maxGroups = 8): void
    {}

    public function convertIPv4ToIPv6(string $addressLiteralIPv4): string
    {}

    /**
     * @param string $addressLiteral
     *
     * @return bool
     */
    protected function checkIPV4Tag($addressLiteral): bool
    {}

    private function addObsoleteWarnings(): void
    {}

    private function addTagWarnings(): void
    {}
}
