<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime;

use Symfony\Component\Mime\Exception\LogicException;
use Symfony\Component\Mime\Part\AbstractPart;
use Symfony\Component\Mime\Part\DataPart;
use Symfony\Component\Mime\Part\File;
use Symfony\Component\Mime\Part\Multipart\AlternativePart;
use Symfony\Component\Mime\Part\Multipart\MixedPart;
use Symfony\Component\Mime\Part\Multipart\RelatedPart;
use Symfony\Component\Mime\Part\TextPart;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
class Email extends Message
{
    public const PRIORITY_HIGHEST = 1;
    public const PRIORITY_HIGH = 2;
    public const PRIORITY_NORMAL = 3;
    public const PRIORITY_LOW = 4;
    public const PRIORITY_LOWEST = 5;

    private const PRIORITY_MAP = [
        self::PRIORITY_HIGHEST => 'Highest',
        self::PRIORITY_HIGH => 'High',
        self::PRIORITY_NORMAL => 'Normal',
        self::PRIORITY_LOW => 'Low',
        self::PRIORITY_LOWEST => 'Lowest',
    ];

    /**
     * @var resource|string|null
     */
    private $text;

    private ?string $textCharset = null;

    /**
     * @var resource|string|null
     */
    private $html;

    private ?string $htmlCharset = null;
    private array $attachments = [];
    private ?AbstractPart $cachedBody = null; // Used to avoid wrong body hash in DKIM signatures with multiple parts (e.g. HTML + TEXT) due to multiple boundaries.

    /**
     * @return $this
     */
    public function subject(string $subject): static
    {}

    public function getSubject(): ?string
    {}

    /**
     * @return $this
     */
    public function date(\DateTimeInterface $dateTime): static
    {}

    public function getDate(): ?\DateTimeImmutable
    {}

    /**
     * @return $this
     */
    public function returnPath(Address|string $address): static
    {}

    public function getReturnPath(): ?Address
    {}

    /**
     * @return $this
     */
    public function sender(Address|string $address): static
    {}

    public function getSender(): ?Address
    {}

    /**
     * @return $this
     */
    public function addFrom(Address|string ...$addresses): static
    {}

    /**
     * @return $this
     */
    public function from(Address|string ...$addresses): static
    {}

    /**
     * @return Address[]
     */
    public function getFrom(): array
    {}

    /**
     * @return $this
     */
    public function addReplyTo(Address|string ...$addresses): static
    {}

    /**
     * @return $this
     */
    public function replyTo(Address|string ...$addresses): static
    {}

    /**
     * @return Address[]
     */
    public function getReplyTo(): array
    {}

    /**
     * @return $this
     */
    public function addTo(Address|string ...$addresses): static
    {}

    /**
     * @return $this
     */
    public function to(Address|string ...$addresses): static
    {}

    /**
     * @return Address[]
     */
    public function getTo(): array
    {}

    /**
     * @return $this
     */
    public function addCc(Address|string ...$addresses): static
    {}

    /**
     * @return $this
     */
    public function cc(Address|string ...$addresses): static
    {}

    /**
     * @return Address[]
     */
    public function getCc(): array
    {}

    /**
     * @return $this
     */
    public function addBcc(Address|string ...$addresses): static
    {}

    /**
     * @return $this
     */
    public function bcc(Address|string ...$addresses): static
    {}

    /**
     * @return Address[]
     */
    public function getBcc(): array
    {}

    /**
     * Sets the priority of this message.
     *
     * The value is an integer where 1 is the highest priority and 5 is the lowest.
     *
     * @return $this
     */
    public function priority(int $priority): static
    {}

    /**
     * Get the priority of this message.
     *
     * The returned value is an integer where 1 is the highest priority and 5
     * is the lowest.
     */
    public function getPriority(): int
    {}

    /**
     * @param resource|string|null $body
     *
     * @return $this
     */
    public function text($body, string $charset = 'utf-8'): static
    {}

    /**
     * @return resource|string|null
     */
    public function getTextBody()
    {}

    public function getTextCharset(): ?string
    {}

    /**
     * @param resource|string|null $body
     *
     * @return $this
     */
    public function html($body, string $charset = 'utf-8'): static
    {}

    /**
     * @return resource|string|null
     */
    public function getHtmlBody()
    {}

    public function getHtmlCharset(): ?string
    {}

    /**
     * @param resource|string $body
     *
     * @return $this
     */
    public function attach($body, ?string $name = null, ?string $contentType = null): static
    {}

    /**
     * @return $this
     */
    public function attachFromPath(string $path, ?string $name = null, ?string $contentType = null): static
    {}

    /**
     * @param resource|string $body
     *
     * @return $this
     */
    public function embed($body, ?string $name = null, ?string $contentType = null): static
    {}

    /**
     * @return $this
     */
    public function embedFromPath(string $path, ?string $name = null, ?string $contentType = null): static
    {}

    /**
     * @return $this
     */
    public function addPart(DataPart $part): static
    {}

    /**
     * @return DataPart[]
     */
    public function getAttachments(): array
    {}

    public function getBody(): AbstractPart
    {}

    public function ensureValidity(): void
    {}

    private function ensureBodyValid(): void
    {}

    /**
     * Generates an AbstractPart based on the raw body of a message.
     *
     * The most "complex" part generated by this method is when there is text and HTML bodies
     * with related images for the HTML part and some attachments:
     *
     * multipart/mixed
     *         |
     *         |------------> multipart/related
     *         |                      |
     *         |                      |------------> multipart/alternative
     *         |                      |                      |
     *         |                      |                       ------------> text/plain (with content)
     *         |                      |                      |
     *         |                      |                       ------------> text/html (with content)
     *         |                      |
     *         |                       ------------> image/png (with content)
     *         |
     *          ------------> application/pdf (with content)
     */
    private function generateBody(): AbstractPart
    {}

    private function prepareParts(): ?array
    {}

    /**
     * @return $this
     */
    private function setHeaderBody(string $type, string $name, $body): static
    {}

    /**
     * @return $this
     */
    private function addListAddressHeaderBody(string $name, array $addresses): static
    {}

    /**
     * @return $this
     */
    private function setListAddressHeaderBody(string $name, array $addresses): static
    {}

    /**
     * @internal
     */
    public function __serialize(): array
    {}

    /**
     * @internal
     */
    public function __unserialize(array $data): void
    {}
}
