<?php

declare(strict_types=1);

namespace GuzzleHttp\Cookie;

use GuzzleHttp\HostIdentity;
use GuzzleHttp\Psr7;

/**
 * Set-Cookie object
 */
class SetCookie
{
    /**
     * @var array
     */
    private const DEFAULTS = [
        'Name' => null,
        'Value' => null,
        'Domain' => null,
        'Path' => '/',
        'Max-Age' => null,
        'Expires' => null,
        'Secure' => false,
        'Discard' => false,
        'HttpOnly' => false,
    ];

    /**
     * @var array Cookie data
     */
    private array $data;

    /**
     * @var bool Whether this cookie was set without a Domain attribute.
     */
    private bool $hostOnly = false;

    /**
     * Create a new SetCookie object from a string.
     *
     * @param string $cookie Set-Cookie header string
     */
    public static function fromString(
        #[\SensitiveParameter]
        string $cookie
    ): self {}

    /**
     * @param array $data Array of cookie data provided by a Cookie parser
     */
    public function __construct(
        #[\SensitiveParameter]
        array $data = []
    ) {}

    public function __toString(): string
    {}

    public function toArray(): array
    {}

    /**
     * Get the cookie name.
     */
    public function getName(): ?string
    {}

    /**
     * Set the cookie name.
     *
     * @param string $name Cookie name
     */
    public function setName(string $name): void
    {}

    /**
     * Get the cookie value.
     */
    public function getValue(): ?string
    {}

    /**
     * Set the cookie value.
     *
     * @param string $value Cookie value
     */
    public function setValue(string $value): void
    {}

    /**
     * Get the domain.
     */
    public function getDomain(): ?string
    {}

    /**
     * Set the domain of the cookie.
     *
     * @param string|null $domain Domain of the cookie
     */
    public function setDomain(?string $domain): void
    {}

    /**
     * Get whether this cookie is scoped to the origin host only.
     */
    public function getHostOnly(): bool
    {}

    /**
     * Set whether this cookie is scoped to the origin host only.
     *
     * @param bool $hostOnly Set to true for host-only cookies
     */
    public function setHostOnly(bool $hostOnly): void
    {}

    /**
     * Get the path.
     */
    public function getPath(): string
    {}

    /**
     * Set the path of the cookie.
     *
     * @param string $path Path of the cookie
     */
    public function setPath(string $path): void
    {}

    /**
     * Maximum lifetime of the cookie in seconds.
     */
    public function getMaxAge(): ?int
    {}

    /**
     * Set the max-age of the cookie.
     *
     * @param int|null $maxAge Max age of the cookie in seconds
     */
    public function setMaxAge(?int $maxAge): void
    {}

    /**
     * The UNIX timestamp when the cookie Expires.
     */
    public function getExpires(): ?int
    {}

    /**
     * Set the unix timestamp for which the cookie will expire.
     *
     * @param int|string|null $timestamp Unix timestamp or any English textual datetime description.
     */
    public function setExpires($timestamp): void
    {}

    /**
     * Get whether or not this is a secure cookie.
     */
    public function getSecure(): bool
    {}

    /**
     * Set whether or not the cookie is secure.
     *
     * @param bool $secure Set to true or false if secure
     */
    public function setSecure(bool $secure): void
    {}

    /**
     * Get whether or not this is a session cookie.
     */
    public function getDiscard(): bool
    {}

    /**
     * Set whether or not this is a session cookie.
     *
     * @param bool $discard Set to true or false if this is a session cookie
     */
    public function setDiscard(bool $discard): void
    {}

    /**
     * Get whether or not this is an HTTP only cookie.
     */
    public function getHttpOnly(): bool
    {}

    /**
     * Set whether or not this is an HTTP only cookie.
     *
     * @param bool $httpOnly Set to true or false if this is HTTP only
     */
    public function setHttpOnly(bool $httpOnly): void
    {}

    /**
     * Check if the cookie matches a path value.
     *
     * A request-path path-matches a given cookie-path if at least one of
     * the following conditions holds:
     *
     * - The cookie-path and the request-path are identical.
     * - The cookie-path is a prefix of the request-path, and the last
     *   character of the cookie-path is %x2F ("/").
     * - The cookie-path is a prefix of the request-path, and the first
     *   character of the request-path that is not included in the cookie-
     *   path is a %x2F ("/") character.
     *
     * @param string $requestPath Path to check against
     */
    public function matchesPath(string $requestPath): bool
    {}

    /**
     * Check if the cookie matches a domain value.
     *
     * @param string $domain Domain to check against
     */
    public function matchesDomain(string $domain): bool
    {}

    /**
     * Check if the cookie is expired.
     */
    public function isExpired(): bool
    {}

    /**
     * Check if the cookie is valid according to RFC 6265.
     *
     * @return string|true Returns true if valid or an error message if invalid
     */
    public function validate()
    {}

    private static function normalizeDomain(string $domain): string
    {}

    private static function maxAgeToExpires(int $maxAge, int $now): int
    {}

    private static function parseNumericInteger(string $value): ?int
    {}

    /**
     * @param mixed[] $data
     */
    private static function validateFieldTypes(
        #[\SensitiveParameter]
        array $data
    ): void {}
}
