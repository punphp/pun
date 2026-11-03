<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use InvalidArgumentException;
use Psr\Http\Message\ServerRequestInterface;
use Psr\Http\Message\StreamInterface;
use Psr\Http\Message\UploadedFileInterface;
use Psr\Http\Message\UriInterface;

/**
 * Server-side HTTP request
 *
 * Extends the Request definition to add methods for accessing incoming data,
 * specifically server parameters, cookies, matched path parameters, query
 * string arguments, body parameters, and upload file information.
 *
 * "Attributes" are discovered via decomposing the request (and usually
 * specifically the URI path), and typically will be injected by the application.
 *
 * Requests are considered immutable; all methods that might change state are
 * implemented such that they retain the internal state of the current
 * message and return a new instance that contains the changed state.
 */
class ServerRequest extends Request implements ServerRequestInterface
{
    private array $attributes = [];

    private array $cookieParams = [];

    /**
     * @var array|object|null
     */
    private $parsedBody;

    private array $queryParams = [];

    private array $serverParams;

    private array $uploadedFiles = [];

    /**
     * @param string                               $method       HTTP method
     * @param string|UriInterface                  $uri          URI
     * @param (string|string[])[]                  $headers      Request headers
     * @param string|resource|StreamInterface|null $body         Request body
     * @param string                               $version      Protocol version
     * @param array                                $serverParams Typically the $_SERVER superglobal
     */
    public function __construct(
        string $method,
        $uri,
        array $headers = [],
        $body = null,
        string $version = '1.1',
        #[\SensitiveParameter]
        array $serverParams = []
    ) {}

    /**
     * Return an UploadedFile instance array.
     *
     * @param array $files An array which respect $_FILES structure
     *
     * @throws InvalidArgumentException for unrecognized values
     */
    public static function normalizeFiles(array $files): array
    {}

    /**
     * Return a ServerRequest populated with superglobals:
     * $_GET
     * $_POST
     * $_COOKIE
     * $_FILES
     * $_SERVER
     */
    public static function fromGlobals(): ServerRequestInterface
    {}

    /**
     * Get a Uri populated with values from $_SERVER.
     */
    public static function getUriFromGlobals(): UriInterface
    {}

    public function getServerParams(): array
    {}

    public function getUploadedFiles(): array
    {}

    public function withUploadedFiles(array $uploadedFiles): ServerRequestInterface
    {}

    public function getCookieParams(): array
    {}

    public function withCookieParams(
        #[\SensitiveParameter]
        array $cookies
    ): ServerRequestInterface {}

    public function getQueryParams(): array
    {}

    public function withQueryParams(array $query): ServerRequestInterface
    {}

    /**
     * @return array|object|null
     */
    public function getParsedBody()
    {}

    public function withParsedBody($data): ServerRequestInterface
    {}

    public function getAttributes(): array
    {}

    /**
     * @return mixed
     */
    public function getAttribute(string $name, $default = null)
    {}

    public function withAttribute(string $name, $value): ServerRequestInterface
    {}

    public function withoutAttribute(string $name): ServerRequestInterface
    {}
}
