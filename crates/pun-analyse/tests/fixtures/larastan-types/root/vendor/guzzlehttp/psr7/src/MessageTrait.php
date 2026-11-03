<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use Psr\Http\Message\MessageInterface;
use Psr\Http\Message\StreamInterface;

/**
 * Trait implementing functionality common to requests and responses.
 */
trait MessageTrait
{
    /** @var string[][] Map of all registered headers, as original name => array of values */
    private array $headers = [];

    /** @var string[] Map of lowercase header name => original name at registration */
    private array $headerNames = [];

    private string $protocol = '1.1';

    private ?StreamInterface $stream = null;

    public function getProtocolVersion(): string
    {}

    /**
     * @return static
     */
    public function withProtocolVersion(string $version): MessageInterface
    {}

    public function getHeaders(): array
    {}

    public function hasHeader(string $name): bool
    {}

    public function getHeader(string $name): array
    {}

    public function getHeaderLine(string $name): string
    {}

    /**
     * @return static
     */
    public function withHeader(string $name, $value): MessageInterface
    {}

    /**
     * @return static
     */
    public function withAddedHeader(string $name, $value): MessageInterface
    {}

    /**
     * @return static
     */
    public function withoutHeader(string $name): MessageInterface
    {}

    public function getBody(): StreamInterface
    {}

    /**
     * @return static
     */
    public function withBody(StreamInterface $body): MessageInterface
    {}

    /**
     * @param (string|string[])[] $headers
     */
    private function setHeaders(array $headers): void
    {}

    /**
     * @param mixed $value
     *
     * @return string[]
     */
    private function normalizeHeaderValue(string $header, $value): array
    {}

    /**
     * Trims whitespace from the header values.
     *
     * Spaces and tabs ought to be excluded by parsers when extracting the field value from a header field.
     *
     * header-field = field-name ":" OWS field-value OWS
     * OWS          = *( SP / HTAB )
     *
     * @param mixed[] $values Header values
     *
     * @return string[] Trimmed header values
     *
     * @see https://datatracker.ietf.org/doc/html/rfc9110#section-5.5
     */
    private function trimAndValidateHeaderValues(string $header, array $values): array
    {}

    /**
     * @see https://datatracker.ietf.org/doc/html/rfc9110#section-5.1
     */
    private function assertHeader(string $header): void
    {}

    private function assertProtocolVersion(string $version): void
    {}

    /**
     * @see https://datatracker.ietf.org/doc/html/rfc9110#section-5.5
     *
     * field-value    = *( field-content / obs-fold )
     * field-content  = field-vchar [ 1*( SP / HTAB ) field-vchar ]
     * field-vchar    = VCHAR / obs-text
     * VCHAR          = %x21-7E
     * obs-text       = %x80-FF
     * obs-fold       = CRLF 1*( SP / HTAB )
     */
    private function assertValue(string $header, string $value): void
    {}
}
