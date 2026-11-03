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

use Symfony\Component\Mime\Address;
use Symfony\Component\Mime\Exception\LogicException;

/**
 * A collection of headers.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
final class Headers
{
    private const UNIQUE_HEADERS = [
        'date', 'from', 'sender', 'reply-to', 'to', 'cc', 'bcc',
        'message-id', 'in-reply-to', 'references', 'subject',
    ];
    private const HEADER_CLASS_MAP = [
        'date' => DateHeader::class,
        'from' => MailboxListHeader::class,
        'sender' => MailboxHeader::class,
        'reply-to' => MailboxListHeader::class,
        'to' => MailboxListHeader::class,
        'cc' => MailboxListHeader::class,
        'bcc' => MailboxListHeader::class,
        'message-id' => IdentificationHeader::class,
        'in-reply-to' => [UnstructuredHeader::class, IdentificationHeader::class], // `In-Reply-To` and `References` are less strict than RFC 2822 (3.6.4) to allow users entering the original email's ...
        'references' => [UnstructuredHeader::class, IdentificationHeader::class], // ... `Message-ID`, even if that is no valid `msg-id`
        'return-path' => PathHeader::class,
    ];

    /**
     * @var HeaderInterface[][]
     */
    private array $headers = [];
    private int $lineLength = 76;

    public function __construct(HeaderInterface ...$headers)
    {}

    public function __clone()
    {}

    public function setMaxLineLength(int $lineLength): void
    {}

    public function getMaxLineLength(): int
    {}

    /**
     * @param array<Address|string> $addresses
     *
     * @return $this
     */
    public function addMailboxListHeader(string $name, array $addresses): static
    {}

    /**
     * @return $this
     */
    public function addMailboxHeader(string $name, Address|string $address): static
    {}

    /**
     * @return $this
     */
    public function addIdHeader(string $name, string|array $ids): static
    {}

    /**
     * @return $this
     */
    public function addPathHeader(string $name, Address|string $path): static
    {}

    /**
     * @return $this
     */
    public function addDateHeader(string $name, \DateTimeInterface $dateTime): static
    {}

    /**
     * @return $this
     */
    public function addTextHeader(string $name, string $value): static
    {}

    /**
     * @return $this
     */
    public function addParameterizedHeader(string $name, string $value, array $params = []): static
    {}

    /**
     * @return $this
     */
    public function addHeader(string $name, mixed $argument, array $more = []): static
    {}

    public function has(string $name): bool
    {}

    /**
     * @return $this
     */
    public function add(HeaderInterface $header): static
    {}

    public function get(string $name): ?HeaderInterface
    {}

    public function all(?string $name = null): iterable
    { yield; }

    public function getNames(): array
    {}

    public function remove(string $name): void
    {}

    public static function isUniqueHeader(string $name): bool
    {}

    /**
     * @throws LogicException if the header name and class are not compatible
     */
    public static function checkHeaderClass(HeaderInterface $header): void
    {}

    public function toString(): string
    {}

    public function toArray(): array
    {}

    public function getHeaderBody(string $name): mixed
    {}

    /**
     * @internal
     */
    public function setHeaderBody(string $type, string $name, mixed $body): void
    {}

    public function getHeaderParameter(string $name, string $parameter): ?string
    {}

    /**
     * @internal
     */
    public function setHeaderParameter(string $name, string $parameter, ?string $value): void
    {}
}
