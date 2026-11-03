<?php

declare(strict_types=1);

namespace GuzzleHttp;

use GuzzleHttp\Cookie\CookieJar;
use GuzzleHttp\Cookie\CookieJarInterface;
use GuzzleHttp\Exception\GuzzleException;
use GuzzleHttp\Exception\InvalidArgumentException;
use GuzzleHttp\Exception\RequestException;
use GuzzleHttp\Handler\CurlShareHandleState;
use GuzzleHttp\Promise as P;
use GuzzleHttp\Promise\PromiseInterface;
use GuzzleHttp\Psr7\DiagnosticValue;
use GuzzleHttp\Psr7\HttpFactory;
use Psr\Http\Message\RequestFactoryInterface;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseFactoryInterface;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Message\StreamFactoryInterface;
use Psr\Http\Message\StreamInterface;
use Psr\Http\Message\UriFactoryInterface;
use Psr\Http\Message\UriInterface;

/**
 * @final
 */
class Client implements ClientInterface, \Psr\Http\Client\ClientInterface
{
    use ClientTrait;
    use NonSerializableTrait;

    /**
     * @var array Default request options
     */
    private array $config;

    /**
     * Clients accept an array of constructor parameters.
     *
     * Here's an example of creating a client using a base_uri and an array of
     * default request options to apply to each request:
     *
     *     $client = new Client([
     *         'base_uri'        => 'http://www.foo.com/1.0/',
     *         'timeout'         => 0,
     *         'allow_redirects' => false,
     *         'proxy'           => '192.168.16.1:10'
     *     ]);
     *
     * Client configuration settings include the following options:
     *
     * - handler: (callable) Function that transfers HTTP requests over the
     *   wire. The function is called with a Psr7\Http\Message\RequestInterface
     *   and array of transfer options, and must return a
     *   GuzzleHttp\Promise\PromiseInterface that is fulfilled with a
     *   Psr7\Http\Message\ResponseInterface on success. If no handler is
     *   provided, a default handler will be created that enables all of the
     *   request options below by attaching all of the default middleware to the
     *   handler.
     * - base_uri: (string|UriInterface) Base URI of the client that is merged
     *   into relative URIs. Can be a string or instance of UriInterface.
     * - transport_sharing: (string|null) Transport sharing mode for the default
     *   handler. Accepts TransportSharing::* or null. Defaults to null.
     * - max_host_connections: (int|null) Maximum concurrent connections per
     *   host, enforced by the default CurlMultiHandler. The default stream
     *   fallback receives the cap as a marker only: it rejects enabled
     *   response streaming ("stream" => true) and does not limit overlapping
     *   buffered calls.
     * - max_total_connections: (int|null) Maximum concurrent connections
     *   overall, enforced by the default CurlMultiHandler. The default stream
     *   fallback receives the cap as a marker only: it rejects enabled
     *   response streaming ("stream" => true) and does not limit overlapping
     *   buffered calls.
     * - multiplex: (string|null) Multiplexing::NONE to disable multiplexing on
     *   the default CurlMultiHandler; the value also becomes the default
     *   "multiplex" request option. Other Multiplexing::* values act as the
     *   default request option only.
     * - **: any request option
     *
     * @param array{
     *     handler?: callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>,
     *     base_uri?: string|UriInterface,
     *     transport_sharing?: string|null,
     *     max_host_connections?: int|null,
     *     max_total_connections?: int|null,
     *     allow_redirects?: bool|array{
     *         max?: int,
     *         strict?: bool,
     *         referer?: bool,
     *         protocols?: non-empty-array<array-key, string>,
     *         on_redirect?: callable(RequestInterface, ResponseInterface, UriInterface): mixed,
     *         track_redirects?: bool
     *     },
     *     auth?: array{
     *         0: string,
     *         1: string,
     *         2?: string|null
     *     }|string|false|null,
     *     body?: resource|string|null|StreamInterface|(callable&object)|\Iterator|\Stringable,
     *     cert?: string|array{
     *         0: string,
     *         1?: string|null
     *     },
     *     cert_type?: string,
     *     connect_timeout?: int|float,
     *     cookies?: bool|CookieJarInterface,
     *     crypto_method?: int,
     *     crypto_method_max?: int,
     *     debug?: bool|resource,
     *     decode_content?: bool|string,
     *     delay?: int|float,
     *     expect?: bool|int,
     *     form_params?: array<array-key, string|int|float|bool|null|array>,
     *     force_ip_resolve?: string,
     *     headers?: array<array-key, string|non-empty-array<array-key, string>>|null,
     *     http_errors?: bool,
     *     idn_conversion?: bool|int|null,
     *     json?: mixed,
     *     multipart?: array<array-key, array{
     *         name: string|int,
     *         contents: mixed,
     *         headers?: array<array-key, string>,
     *         filename?: string
     *     }>,
     *     multiplex?: string,
     *     on_headers?: callable(ResponseInterface, RequestInterface): mixed,
     *     on_stats?: callable(TransferStats): mixed,
     *     on_trailers?: callable(array<string, list<string>>, ResponseInterface, RequestInterface): mixed,
     *     progress?: callable(int, int, int, int): mixed,
     *     protocols?: non-empty-array<array-key, string>,
     *     proxy?: string|array{
     *         http?: string|null,
     *         https?: string|null,
     *         no?: string|array<array-key, string>|null
     *     },
     *     query?: array<array-key, mixed>|string,
     *     read_timeout?: int|float,
     *     retries?: int,
     *     request_factory?: RequestFactoryInterface,
     *     response_factory?: ResponseFactoryInterface,
     *     sink?: resource|string|StreamInterface,
     *     ssl_key?: string|array{
     *         0: string,
     *         1?: string|null
     *     },
     *     ssl_key_type?: string,
     *     stream?: bool,
     *     stream_factory?: StreamFactoryInterface,
     *     stream_context?: array<array-key, mixed>,
     *     synchronous?: bool,
     *     timeout?: int|float,
     *     uri_factory?: UriFactoryInterface,
     *     verify?: bool|string,
     *     version?: string|int|float,
     *     curl?: array<int|string, mixed>,
     *     ...
     * } $config Client configuration settings and default request options.
     *
     * @see RequestOptions for a list of available request options.
     */
    public function __construct(
        #[\SensitiveParameter]
        array $config = []
    ) {}

    /**
     * Asynchronously send an HTTP request.
     *
     * @param array{
     *     base_uri?: string|UriInterface,
     *     allow_redirects?: bool|array{
     *         max?: int,
     *         strict?: bool,
     *         referer?: bool,
     *         protocols?: non-empty-array<array-key, string>,
     *         on_redirect?: callable(RequestInterface, ResponseInterface, UriInterface): mixed,
     *         track_redirects?: bool
     *     },
     *     auth?: array{
     *         0: string,
     *         1: string,
     *         2?: string|null
     *     }|string|false|null,
     *     body?: resource|string|null|StreamInterface|(callable&object)|\Iterator|\Stringable,
     *     cert?: string|array{
     *         0: string,
     *         1?: string|null
     *     },
     *     cert_type?: string,
     *     connect_timeout?: int|float,
     *     cookies?: false|CookieJarInterface,
     *     crypto_method?: int,
     *     crypto_method_max?: int,
     *     debug?: bool|resource,
     *     decode_content?: bool|string,
     *     delay?: int|float,
     *     expect?: bool|int,
     *     form_params?: array<array-key, string|int|float|bool|null|array>,
     *     force_ip_resolve?: string,
     *     headers?: array<array-key, string|non-empty-array<array-key, string>>|null,
     *     http_errors?: bool,
     *     idn_conversion?: bool|int|null,
     *     json?: mixed,
     *     multipart?: array<array-key, array{
     *         name: string|int,
     *         contents: mixed,
     *         headers?: array<array-key, string>,
     *         filename?: string
     *     }>,
     *     multiplex?: string,
     *     on_headers?: callable(ResponseInterface, RequestInterface): mixed,
     *     on_stats?: callable(TransferStats): mixed,
     *     on_trailers?: callable(array<string, list<string>>, ResponseInterface, RequestInterface): mixed,
     *     progress?: callable(int, int, int, int): mixed,
     *     protocols?: non-empty-array<array-key, string>,
     *     proxy?: string|array{
     *         http?: string|null,
     *         https?: string|null,
     *         no?: string|array<array-key, string>|null
     *     },
     *     query?: array<array-key, mixed>|string,
     *     read_timeout?: int|float,
     *     retries?: int,
     *     request_factory?: RequestFactoryInterface,
     *     response_factory?: ResponseFactoryInterface,
     *     sink?: resource|string|StreamInterface,
     *     ssl_key?: string|array{
     *         0: string,
     *         1?: string|null
     *     },
     *     ssl_key_type?: string,
     *     stream?: bool,
     *     stream_factory?: StreamFactoryInterface,
     *     stream_context?: array<array-key, mixed>,
     *     synchronous?: bool,
     *     timeout?: int|float,
     *     uri_factory?: UriFactoryInterface,
     *     verify?: bool|string,
     *     version?: string|int|float,
     *     curl?: array<int|string, mixed>,
     *     ...
     * } $options Request options to apply to the given request and to the transfer. See {@see RequestOptions}.
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    public function sendAsync(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options = []
    ): PromiseInterface {}

    /**
     * Send an HTTP request.
     *
     * @param array{
     *     base_uri?: string|UriInterface,
     *     allow_redirects?: bool|array{
     *         max?: int,
     *         strict?: bool,
     *         referer?: bool,
     *         protocols?: non-empty-array<array-key, string>,
     *         on_redirect?: callable(RequestInterface, ResponseInterface, UriInterface): mixed,
     *         track_redirects?: bool
     *     },
     *     auth?: array{
     *         0: string,
     *         1: string,
     *         2?: string|null
     *     }|string|false|null,
     *     body?: resource|string|null|StreamInterface|(callable&object)|\Iterator|\Stringable,
     *     cert?: string|array{
     *         0: string,
     *         1?: string|null
     *     },
     *     cert_type?: string,
     *     connect_timeout?: int|float,
     *     cookies?: false|CookieJarInterface,
     *     crypto_method?: int,
     *     crypto_method_max?: int,
     *     debug?: bool|resource,
     *     decode_content?: bool|string,
     *     delay?: int|float,
     *     expect?: bool|int,
     *     form_params?: array<array-key, string|int|float|bool|null|array>,
     *     force_ip_resolve?: string,
     *     headers?: array<array-key, string|non-empty-array<array-key, string>>|null,
     *     http_errors?: bool,
     *     idn_conversion?: bool|int|null,
     *     json?: mixed,
     *     multipart?: array<array-key, array{
     *         name: string|int,
     *         contents: mixed,
     *         headers?: array<array-key, string>,
     *         filename?: string
     *     }>,
     *     multiplex?: string,
     *     on_headers?: callable(ResponseInterface, RequestInterface): mixed,
     *     on_stats?: callable(TransferStats): mixed,
     *     on_trailers?: callable(array<string, list<string>>, ResponseInterface, RequestInterface): mixed,
     *     progress?: callable(int, int, int, int): mixed,
     *     protocols?: non-empty-array<array-key, string>,
     *     proxy?: string|array{
     *         http?: string|null,
     *         https?: string|null,
     *         no?: string|array<array-key, string>|null
     *     },
     *     query?: array<array-key, mixed>|string,
     *     read_timeout?: int|float,
     *     retries?: int,
     *     request_factory?: RequestFactoryInterface,
     *     response_factory?: ResponseFactoryInterface,
     *     sink?: resource|string|StreamInterface,
     *     ssl_key?: string|array{
     *         0: string,
     *         1?: string|null
     *     },
     *     ssl_key_type?: string,
     *     stream?: bool,
     *     stream_factory?: StreamFactoryInterface,
     *     stream_context?: array<array-key, mixed>,
     *     synchronous?: bool,
     *     timeout?: int|float,
     *     uri_factory?: UriFactoryInterface,
     *     verify?: bool|string,
     *     version?: string|int|float,
     *     curl?: array<int|string, mixed>,
     *     ...
     * } $options Request options to apply to the given request and to the transfer. See {@see RequestOptions}.
     *
     * @throws GuzzleException
     */
    public function send(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options = []
    ): ResponseInterface {}

    /**
     * The HttpClient PSR (PSR-18) specifies this method.
     *
     * {@inheritDoc}
     */
    public function sendRequest(
        #[\SensitiveParameter]
        RequestInterface $request
    ): ResponseInterface {}

    /**
     * Create and send an asynchronous HTTP request.
     *
     * Use an absolute path to override the base path of the client, or a
     * relative path to append to the base path of the client. The URL can
     * contain the query string as well.
     *
     * @param string              $method HTTP method
     * @param string|UriInterface $uri    URI object or string.
     * @param array{
     *     base_uri?: string|UriInterface,
     *     allow_redirects?: bool|array{
     *         max?: int,
     *         strict?: bool,
     *         referer?: bool,
     *         protocols?: non-empty-array<array-key, string>,
     *         on_redirect?: callable(RequestInterface, ResponseInterface, UriInterface): mixed,
     *         track_redirects?: bool
     *     },
     *     auth?: array{
     *         0: string,
     *         1: string,
     *         2?: string|null
     *     }|string|false|null,
     *     body?: resource|string|null|StreamInterface|(callable&object)|\Iterator|\Stringable,
     *     cert?: string|array{
     *         0: string,
     *         1?: string|null
     *     },
     *     cert_type?: string,
     *     connect_timeout?: int|float,
     *     cookies?: false|CookieJarInterface,
     *     crypto_method?: int,
     *     crypto_method_max?: int,
     *     debug?: bool|resource,
     *     decode_content?: bool|string,
     *     delay?: int|float,
     *     expect?: bool|int,
     *     form_params?: array<array-key, string|int|float|bool|null|array>,
     *     force_ip_resolve?: string,
     *     headers?: array<array-key, string|non-empty-array<array-key, string>>|null,
     *     http_errors?: bool,
     *     idn_conversion?: bool|int|null,
     *     json?: mixed,
     *     multipart?: array<array-key, array{
     *         name: string|int,
     *         contents: mixed,
     *         headers?: array<array-key, string>,
     *         filename?: string
     *     }>,
     *     multiplex?: string,
     *     on_headers?: callable(ResponseInterface, RequestInterface): mixed,
     *     on_stats?: callable(TransferStats): mixed,
     *     on_trailers?: callable(array<string, list<string>>, ResponseInterface, RequestInterface): mixed,
     *     progress?: callable(int, int, int, int): mixed,
     *     protocols?: non-empty-array<array-key, string>,
     *     proxy?: string|array{
     *         http?: string|null,
     *         https?: string|null,
     *         no?: string|array<array-key, string>|null
     *     },
     *     query?: array<array-key, mixed>|string,
     *     read_timeout?: int|float,
     *     retries?: int,
     *     request_factory?: RequestFactoryInterface,
     *     response_factory?: ResponseFactoryInterface,
     *     sink?: resource|string|StreamInterface,
     *     ssl_key?: string|array{
     *         0: string,
     *         1?: string|null
     *     },
     *     ssl_key_type?: string,
     *     stream?: bool,
     *     stream_factory?: StreamFactoryInterface,
     *     stream_context?: array<array-key, mixed>,
     *     synchronous?: bool,
     *     timeout?: int|float,
     *     uri_factory?: UriFactoryInterface,
     *     verify?: bool|string,
     *     version?: string|int|float,
     *     curl?: array<int|string, mixed>,
     *     ...
     * } $options Request options to apply. See {@see RequestOptions}.
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    public function requestAsync(
        string $method,
        $uri = '',
        #[\SensitiveParameter]
        array $options = []
    ): PromiseInterface {}

    /**
     * Create and send an HTTP request.
     *
     * Use an absolute path to override the base path of the client, or a
     * relative path to append to the base path of the client. The URL can
     * contain the query string as well.
     *
     * @param string              $method HTTP method.
     * @param string|UriInterface $uri    URI object or string.
     * @param array{
     *     base_uri?: string|UriInterface,
     *     allow_redirects?: bool|array{
     *         max?: int,
     *         strict?: bool,
     *         referer?: bool,
     *         protocols?: non-empty-array<array-key, string>,
     *         on_redirect?: callable(RequestInterface, ResponseInterface, UriInterface): mixed,
     *         track_redirects?: bool
     *     },
     *     auth?: array{
     *         0: string,
     *         1: string,
     *         2?: string|null
     *     }|string|false|null,
     *     body?: resource|string|null|StreamInterface|(callable&object)|\Iterator|\Stringable,
     *     cert?: string|array{
     *         0: string,
     *         1?: string|null
     *     },
     *     cert_type?: string,
     *     connect_timeout?: int|float,
     *     cookies?: false|CookieJarInterface,
     *     crypto_method?: int,
     *     crypto_method_max?: int,
     *     debug?: bool|resource,
     *     decode_content?: bool|string,
     *     delay?: int|float,
     *     expect?: bool|int,
     *     form_params?: array<array-key, string|int|float|bool|null|array>,
     *     force_ip_resolve?: string,
     *     headers?: array<array-key, string|non-empty-array<array-key, string>>|null,
     *     http_errors?: bool,
     *     idn_conversion?: bool|int|null,
     *     json?: mixed,
     *     multipart?: array<array-key, array{
     *         name: string|int,
     *         contents: mixed,
     *         headers?: array<array-key, string>,
     *         filename?: string
     *     }>,
     *     multiplex?: string,
     *     on_headers?: callable(ResponseInterface, RequestInterface): mixed,
     *     on_stats?: callable(TransferStats): mixed,
     *     on_trailers?: callable(array<string, list<string>>, ResponseInterface, RequestInterface): mixed,
     *     progress?: callable(int, int, int, int): mixed,
     *     protocols?: non-empty-array<array-key, string>,
     *     proxy?: string|array{
     *         http?: string|null,
     *         https?: string|null,
     *         no?: string|array<array-key, string>|null
     *     },
     *     query?: array<array-key, mixed>|string,
     *     read_timeout?: int|float,
     *     retries?: int,
     *     request_factory?: RequestFactoryInterface,
     *     response_factory?: ResponseFactoryInterface,
     *     sink?: resource|string|StreamInterface,
     *     ssl_key?: string|array{
     *         0: string,
     *         1?: string|null
     *     },
     *     ssl_key_type?: string,
     *     stream?: bool,
     *     stream_factory?: StreamFactoryInterface,
     *     stream_context?: array<array-key, mixed>,
     *     synchronous?: bool,
     *     timeout?: int|float,
     *     uri_factory?: UriFactoryInterface,
     *     verify?: bool|string,
     *     version?: string|int|float,
     *     curl?: array<int|string, mixed>,
     *     ...
     * } $options Request options to apply. See {@see RequestOptions}.
     *
     * @throws GuzzleException
     */
    public function request(
        string $method,
        $uri = '',
        #[\SensitiveParameter]
        array $options = []
    ): ResponseInterface {}

    /**
     * Get a client configuration option.
     *
     * These options include default request options of the client, a "handler"
     * (if utilized by the concrete client), and a "base_uri" if utilized by the
     * concrete client.
     *
     * @param string|null $option The config option to retrieve.
     *
     * @return ($option is null ? array<string, mixed> : mixed)
     */
    public function getConfig(?string $option = null)
    {}

    private function buildUri(
        UriInterface $uri,
        #[\SensitiveParameter]
        array $config
    ): UriInterface {}

    /**
     * Whether to preserve an existing Host header when the URI changes.
     *
     * A header matching the current URI carries no explicit override and is
     * regenerated after base URI resolution or IDN conversion. Other values
     * are preserved as deliberate overrides, as PSR-7 requires.
     */
    private static function shouldPreserveHost(RequestInterface $request): bool
    {}

    /**
     * @param mixed $factory
     */
    private static function requireRequestFactory($factory): RequestFactoryInterface
    {}

    /**
     * @param mixed $factory
     */
    private static function requireResponseFactory($factory): ResponseFactoryInterface
    {}

    /**
     * @param mixed $factory
     */
    private static function requireUriFactory($factory): UriFactoryInterface
    {}

    /**
     * @param mixed $factory
     */
    private static function requireStreamFactory($factory): StreamFactoryInterface
    {}

    /**
     * @param mixed $uri
     */
    private static function createUri($uri, UriFactoryInterface $uriFactory): UriInterface
    {}

    /**
     * @param mixed $body
     */
    private static function createBodyStream($body, StreamFactoryInterface $streamFactory): StreamInterface
    {}

    /**
     * Configures the default options for a client.
     */
    private function configureDefaults(
        #[\SensitiveParameter]
        array $config
    ): void {}

    /**
     * Merges default options into the array.
     *
     * @param array $options Options to modify by reference
     */
    private function prepareDefaults(
        #[\SensitiveParameter]
        array $options
    ): array {}

    private static function assertRequestOptionTypes(
        #[\SensitiveParameter]
        array $options
    ): void {}

    private static function assertAllowRedirectsOptionTypes(array $allowRedirects): void
    {}

    /**
     * @param array<array-key, mixed> $auth
     */
    private static function assertAuthOptionTypes(
        #[\SensitiveParameter]
        array $auth
    ): void {}

    /**
     * @param mixed $value
     */
    private static function assertFormParamTypes($value): void
    {}

    private static function assertFormParamArray(array $values, string $path): bool
    {}

    private static function assertFiniteFloats(array $values, string $option): void
    {}

    /**
     * @param mixed $headers
     */
    private static function assertHeaderOptionTypes(
        #[\SensitiveParameter]
        $headers
    ): void {}

    /**
     * @param mixed $multipart
     */
    private static function assertMultipartOptionTypes($multipart): void
    {}

    private static function assertProxyOptionTypes(
        #[\SensitiveParameter]
        array $options
    ): void {}

    private static function assertTlsFileOptionTypes(
        #[\SensitiveParameter]
        array $options,
        string $option
    ): void {}

    private static function assertIfPresentAndNotArray(
        #[\SensitiveParameter]
        array $options,
        string $option,
        string $expected
    ): void {}

    private static function assertIfPresentAndNotBool(
        #[\SensitiveParameter]
        array $options,
        string $option,
        ?string $path = null
    ): void {}

    private static function assertValidMultiplex(
        #[\SensitiveParameter]
        array $options
    ): void {}

    private static function assertIfPresentAndNotBoolOrInt(
        #[\SensitiveParameter]
        array $options,
        string $option
    ): void {}

    private static function assertIfPresentAndNotBoolOrResource(
        #[\SensitiveParameter]
        array $options,
        string $option
    ): void {}

    private static function assertIfPresentAndNotBoolOrString(
        #[\SensitiveParameter]
        array $options,
        string $option
    ): void {}

    private static function assertIfPresentAndNotCallable(
        #[\SensitiveParameter]
        array $options,
        string $option,
        ?string $path = null
    ): void {}

    private static function assertIfPresentAndNotInt(
        #[\SensitiveParameter]
        array $options,
        string $option,
        ?string $path = null
    ): void {}

    private static function assertIfPresentAndNotNumber(
        #[\SensitiveParameter]
        array $options,
        string $option
    ): void {}

    /**
     * @param array<array-key, mixed> $options
     */
    private static function assertIfPresentAndNotFiniteNonNegativeNumber(
        #[\SensitiveParameter]
        array $options,
        string $option
    ): void {}

    /**
     * @param array<array-key, mixed> $options
     */
    private static function assertIfPresentAndNotForceIpResolve(
        #[\SensitiveParameter]
        array $options
    ): void {}

    private static function assertIfPresentAndNotString(
        #[\SensitiveParameter]
        array $options,
        string $option
    ): void {}

    /**
     * @param array<array-key, mixed> $options
     */
    private static function assertIfPresentAndNotProtocolArray(
        #[\SensitiveParameter]
        array $options,
        string $option,
        ?string $path = null
    ): void {}

    private static function assertIfPresentAndNotStringOrNumber(
        #[\SensitiveParameter]
        array $options,
        string $option
    ): void {}

    /**
     * @param mixed $value
     */
    private static function invalidRequestOptionType(
        string $option,
        string $expected,
        #[\SensitiveParameter]
        $value
    ): void {}

    /**
     * Transfers the given request and applies request options.
     *
     * The URI of the request is not modified and the request options are used
     * as-is without merging in default options.
     *
     * @param array $options See {@see RequestOptions}.
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private function transfer(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ): PromiseInterface {}

    /**
     * Applies the array of request options to a request.
     */
    private function applyOptions(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array &$options
    ): RequestInterface {}

    /**
     * @param string|int|float $version
     */
    private static function normalizeProtocolVersion($version): string
    {}

    private static function assertProtocolVersion(string $version): void
    {}

    private static function assertRequestProtocolVersion(
        #[\SensitiveParameter]
        RequestInterface $request
    ): void {}

    private static function getMultipartContentType(Psr7\MultipartStream $body): string
    {}

    /**
     * Return an InvalidArgumentException with pre-set message.
     */
    private function invalidBody(): InvalidArgumentException
    {}
}
