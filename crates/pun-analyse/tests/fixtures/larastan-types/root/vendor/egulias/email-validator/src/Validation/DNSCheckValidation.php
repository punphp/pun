<?php

namespace Egulias\EmailValidator\Validation;

use Egulias\EmailValidator\EmailLexer;
use Egulias\EmailValidator\Result\InvalidEmail;
use Egulias\EmailValidator\Result\Reason\DomainAcceptsNoMail;
use Egulias\EmailValidator\Result\Reason\LocalOrReservedDomain;
use Egulias\EmailValidator\Result\Reason\NoDNSRecord as ReasonNoDNSRecord;
use Egulias\EmailValidator\Result\Reason\UnableToGetDNSRecord;
use Egulias\EmailValidator\Warning\NoDNSMXRecord;
use Egulias\EmailValidator\Warning\Warning;

class DNSCheckValidation implements EmailValidation
{

    /**
     * Reserved Top Level DNS Names (https://tools.ietf.org/html/rfc2606#section-2),
     * mDNS and private DNS Namespaces (https://tools.ietf.org/html/rfc6762#appendix-G)
     * 
     * @var string[]
     */
    public const RESERVED_DNS_TOP_LEVEL_NAMES = [
        // Reserved Top Level DNS Names
        'test',
        'example',
        'invalid',
        'localhost',

        // mDNS
        'local',

        // Private DNS Namespaces
        'intranet',
        'internal',
        'private',
        'corp',
        'home',
        'lan',
    ];

    /**
     * @var Warning[]
     */
    private $warnings = [];

    /**
     * @var InvalidEmail|null
     */
    private $error;

    /**
     * @var array
     */
    private $mxRecords = [];

    /**
     * @var DNSGetRecordWrapper
     */
    private $dnsGetRecord;

    public function __construct(?DNSGetRecordWrapper $dnsGetRecord = null)
    {}

    public function isValid(string $email, EmailLexer $emailLexer): bool
    {}

    public function getError(): ?InvalidEmail
    {}

    /**
     * @return Warning[]
     */
    public function getWarnings(): array
    {}

    /**
     * @param string $host
     *
     * @return bool
     */
    protected function checkDns($host)
    {}


    /**
     * Validate the DNS records for given host.
     *
     * @param string $host A set of DNS records in the format returned by dns_get_record.
     *
     * @return bool True on success.
     */
    private function validateDnsRecords($host): bool
    {}

    /**
     * Validate an MX record
     *
     * @param array $dnsRecord Given DNS record.
     *
     * @return bool True if valid.
     */
    private function validateMxRecord($dnsRecord): bool
    {}
}
