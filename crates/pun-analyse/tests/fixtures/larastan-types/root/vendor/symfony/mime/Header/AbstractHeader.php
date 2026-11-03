<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime\Header;

use Symfony\Component\Mime\Encoder\QpMimeHeaderEncoder;
use Symfony\Component\Mime\Exception\RfcComplianceException;

/**
 * An abstract base MIME Header.
 *
 * @author Chris Corbyn
 */
abstract class AbstractHeader implements HeaderInterface
{
    public const PHRASE_PATTERN = '(?:(?:(?:(?:(?:(?:(?:[ \t]*(?:\r\n))?[ \t])?(\((?:(?:(?:[ \t]*(?:\r\n))?[ \t])|(?:(?:[\x01-\x08\x0B\x0C\x0E-\x19\x7F]|[\x21-\x27\x2A-\x5B\x5D-\x7E])|(?:\\[\x00-\x08\x0B\x0C\x0E-\x7F])|(?1)))*(?:(?:[ \t]*(?:\r\n))?[ \t])?\)))*(?:(?:(?:(?:[ \t]*(?:\r\n))?[ \t])?(\((?:(?:(?:[ \t]*(?:\r\n))?[ \t])|(?:(?:[\x01-\x08\x0B\x0C\x0E-\x19\x7F]|[\x21-\x27\x2A-\x5B\x5D-\x7E])|(?:\\[\x00-\x08\x0B\x0C\x0E-\x7F])|(?1)))*(?:(?:[ \t]*(?:\r\n))?[ \t])?\)))|(?:(?:[ \t]*(?:\r\n))?[ \t])))?[a-zA-Z0-9!#\$%&\'\*\+\-\/=\?\^_`\{\}\|~]+(?:(?:(?:(?:[ \t]*(?:\r\n))?[ \t])?(\((?:(?:(?:[ \t]*(?:\r\n))?[ \t])|(?:(?:[\x01-\x08\x0B\x0C\x0E-\x19\x7F]|[\x21-\x27\x2A-\x5B\x5D-\x7E])|(?:\\[\x00-\x08\x0B\x0C\x0E-\x7F])|(?1)))*(?:(?:[ \t]*(?:\r\n))?[ \t])?\)))*(?:(?:(?:(?:[ \t]*(?:\r\n))?[ \t])?(\((?:(?:(?:[ \t]*(?:\r\n))?[ \t])|(?:(?:[\x01-\x08\x0B\x0C\x0E-\x19\x7F]|[\x21-\x27\x2A-\x5B\x5D-\x7E])|(?:\\[\x00-\x08\x0B\x0C\x0E-\x7F])|(?1)))*(?:(?:[ \t]*(?:\r\n))?[ \t])?\)))|(?:(?:[ \t]*(?:\r\n))?[ \t])))?)|(?:(?:(?:(?:(?:[ \t]*(?:\r\n))?[ \t])?(\((?:(?:(?:[ \t]*(?:\r\n))?[ \t])|(?:(?:[\x01-\x08\x0B\x0C\x0E-\x19\x7F]|[\x21-\x27\x2A-\x5B\x5D-\x7E])|(?:\\[\x00-\x08\x0B\x0C\x0E-\x7F])|(?1)))*(?:(?:[ \t]*(?:\r\n))?[ \t])?\)))*(?:(?:(?:(?:[ \t]*(?:\r\n))?[ \t])?(\((?:(?:(?:[ \t]*(?:\r\n))?[ \t])|(?:(?:[\x01-\x08\x0B\x0C\x0E-\x19\x7F]|[\x21-\x27\x2A-\x5B\x5D-\x7E])|(?:\\[\x00-\x08\x0B\x0C\x0E-\x7F])|(?1)))*(?:(?:[ \t]*(?:\r\n))?[ \t])?\)))|(?:(?:[ \t]*(?:\r\n))?[ \t])))?"((?:(?:[ \t]*(?:\r\n))?[ \t])?(?:(?:[\x01-\x08\x0B\x0C\x0E-\x19\x7F]|[\x21\x23-\x5B\x5D-\x7E])|(?:\\[\x00-\x08\x0B\x0C\x0E-\x7F])))*(?:(?:[ \t]*(?:\r\n))?[ \t])?"(?:(?:(?:(?:[ \t]*(?:\r\n))?[ \t])?(\((?:(?:(?:[ \t]*(?:\r\n))?[ \t])|(?:(?:[\x01-\x08\x0B\x0C\x0E-\x19\x7F]|[\x21-\x27\x2A-\x5B\x5D-\x7E])|(?:\\[\x00-\x08\x0B\x0C\x0E-\x7F])|(?1)))*(?:(?:[ \t]*(?:\r\n))?[ \t])?\)))*(?:(?:(?:(?:[ \t]*(?:\r\n))?[ \t])?(\((?:(?:(?:[ \t]*(?:\r\n))?[ \t])|(?:(?:[\x01-\x08\x0B\x0C\x0E-\x19\x7F]|[\x21-\x27\x2A-\x5B\x5D-\x7E])|(?:\\[\x00-\x08\x0B\x0C\x0E-\x7F])|(?1)))*(?:(?:[ \t]*(?:\r\n))?[ \t])?\)))|(?:(?:[ \t]*(?:\r\n))?[ \t])))?))+?)';

    private static QpMimeHeaderEncoder $encoder;

    private string $name;
    private int $lineLength = 76;
    private ?string $lang = null;
    private string $charset = 'utf-8';

    public function __construct(string $name)
    {}

    public function setCharset(string $charset): void
    {}

    public function getCharset(): ?string
    {}

    /**
     * Set the language used in this Header.
     *
     * For example, for US English, 'en-us'.
     */
    public function setLanguage(string $lang): void
    {}

    public function getLanguage(): ?string
    {}

    public function getName(): string
    {}

    public function setMaxLineLength(int $lineLength): void
    {}

    public function getMaxLineLength(): int
    {}

    public function toString(): string
    {}

    /**
     * Produces a compliant, formatted RFC 2822 'phrase' based on the string given.
     *
     * @param string $string  as displayed
     * @param bool   $shorten the first line to make remove for header name
     */
    protected function createPhrase(HeaderInterface $header, string $string, string $charset, bool $shorten = false): string
    {}

    /**
     * Encode needed word tokens within a string of input.
     */
    protected function encodeWords(HeaderInterface $header, string $input, int $usedLength = -1): string
    {}

    protected function tokenNeedsEncoding(string $token): bool
    {}

    /**
     * Splits a string into tokens in blocks of words which can be encoded quickly.
     *
     * @return string[]
     */
    protected function getEncodableWordTokens(string $string): array
    {}

    /**
     * Get a token as an encoded word for safe insertion into headers.
     */
    protected function getTokenAsEncodedWord(string $token, int $firstLineOffset = 0): string
    {}

    /**
     * Generates tokens from the given string which include CRLF as individual tokens.
     *
     * @return string[]
     */
    protected function generateTokenLines(string $token): array
    {}

    /**
     * Generate a list of all tokens in the final header.
     */
    protected function toTokens(?string $string = null): array
    {}

    /**
     * Takes an array of tokens which appear in the header and turns them into
     * an RFC 2822 compliant string, adding FWSP where needed.
     *
     * @param string[] $tokens
     */
    private function tokensToString(array $tokens): string
    {}
}
