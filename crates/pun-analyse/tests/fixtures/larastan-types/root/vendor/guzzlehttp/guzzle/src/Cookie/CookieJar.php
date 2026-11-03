<?php

declare(strict_types=1);

namespace GuzzleHttp\Cookie;

use GuzzleHttp\HostIdentity;
use GuzzleHttp\Psr7;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;

/**
 * Cookie jar that stores cookies as an array
 */
class CookieJar implements CookieJarInterface
{
    private const MAX_SET_COOKIE_FIELD_LENGTH = 8190;
    private const MAX_SET_COOKIE_FIELDS = 50;
    private const MAX_REQUEST_COOKIES = 150;
    private const MAX_COOKIE_HEADER_LENGTH = 8190;

    /**
     * @var SetCookie[] Loaded cookie data
     */
    private array $cookies = [];

    private bool $strictMode;

    /**
     * @param bool  $strictMode  Set to true to throw exceptions when invalid
     *                           cookies are added to the cookie jar.
     * @param array $cookieArray Array of SetCookie objects or a hash of
     *                           arrays that can be used with the SetCookie
     *                           constructor
     */
    public function __construct(
        bool $strictMode = false,
        #[\SensitiveParameter]
        array $cookieArray = []
    ) {}

    /**
     * Create a new Cookie jar from an associative array and domain.
     *
     * @param array  $cookies Cookies to create the jar from
     * @param string $domain  Domain to set the cookies to
     */
    public static function fromArray(
        #[\SensitiveParameter]
        array $cookies,
        string $domain
    ): self {}

    /**
     * Evaluate if this cookie should be persisted to storage
     * that survives between requests.
     *
     * @param SetCookie $cookie              Being evaluated.
     * @param bool      $allowSessionCookies If we should persist session cookies
     */
    public static function shouldPersist(SetCookie $cookie, bool $allowSessionCookies = false): bool
    {}

    /**
     * Finds and returns the cookie based on the name
     *
     * @param string $name cookie name to search for
     *
     * @return SetCookie|null cookie that was found or null if not found
     */
    public function getCookieByName(string $name): ?SetCookie
    {}

    public function toArray(): array
    {}

    public function clear(?string $domain = null, ?string $path = null, ?string $name = null): void
    {}

    public function clearSessionCookies(): void
    {}

    public function setCookie(
        #[\SensitiveParameter]
        SetCookie $cookie
    ): bool {}

    public function count(): int
    {}

    /**
     * @return \ArrayIterator<int, SetCookie>
     */
    public function getIterator(): \ArrayIterator
    {}

    public function extractCookies(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        ResponseInterface $response
    ): void {}

    private function overlaysSecureCookie(SetCookie $cookie): bool
    {}

    private static function isSecureCookieOverlay(SetCookie $cookie, SetCookie $stored): bool
    {}

    /**
     * Mirrors SetCookie::fromString()'s splitting because parsed cookies cannot
     * distinguish an absent Path attribute from a defaulted path.
     */
    private static function hasPathAttribute(string $header): bool
    {}

    /**
     * Computes cookie path following RFC 6265 section 5.1.4
     *
     * @see https://datatracker.ietf.org/doc/html/rfc6265#section-5.1.4
     */
    private function getCookiePathFromRequest(RequestInterface $request): string
    {}

    public function withCookieHeader(
        #[\SensitiveParameter]
        RequestInterface $request
    ): RequestInterface {}

    /**
     * If a cookie already exists and the server asks to set it again with a
     * null value, the cookie must be deleted.
     */
    private function removeCookieIfEmpty(SetCookie $cookie): void
    {}

    private function removeCookie(SetCookie $cookie): void
    {}
}
