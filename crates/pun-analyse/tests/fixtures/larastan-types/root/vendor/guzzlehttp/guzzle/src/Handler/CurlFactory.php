<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\Exception\ConnectException;
use GuzzleHttp\Exception\ConnectTimeoutException;
use GuzzleHttp\Exception\InvalidArgumentException;
use GuzzleHttp\Exception\NetworkException;
use GuzzleHttp\Exception\NetworkTimeoutException;
use GuzzleHttp\Exception\RequestException;
use GuzzleHttp\Exception\ResponseException;
use GuzzleHttp\Exception\ResponseTimeoutException;
use GuzzleHttp\Exception\ResponseTransferException;
use GuzzleHttp\Multiplexing;
use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\Promise as P;
use GuzzleHttp\Promise\PromiseInterface;
use GuzzleHttp\ProxyOptions;
use GuzzleHttp\Psr7;
use GuzzleHttp\Psr7\Exception\TimeoutException;
use GuzzleHttp\Psr7\FnStream;
use GuzzleHttp\Psr7\HttpFactory;
use GuzzleHttp\Psr7\LazyOpenStream;
use GuzzleHttp\RequestOptions;
use GuzzleHttp\TransferStats;
use GuzzleHttp\TransportSharing;
use GuzzleHttp\Utils;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Message\StreamFactoryInterface;
use Psr\Http\Message\StreamInterface;
use Psr\Http\Message\UriInterface;

/**
 * Creates curl resources from a request
 */
final class CurlFactory implements CurlFactoryInterface
{
    use NonSerializableTrait;

    private const DELEGATED_PROXY_TUNNEL_OWNER = 'proxy-tunnel:delegated-to-libcurl';

    /**
     * String-valued proxy credential cURL options whose values feed the
     * connection-reuse section signatures. Stringable values are cast
     * exactly once, before signature computation, so the signature and
     * ext-curl observe the same string; a stateful __toString() could
     * otherwise produce one value for the signature and a different one on
     * the wire, giving two credentials the same section. Numeric options
     * (CURLOPT_PROXYTYPE, CURLOPT_PROXY_SSLVERSION) and blob options
     * (CURLOPT_PROXY_SSLCERT_BLOB) are deliberately excluded.
     */
    private const STRINGABLE_PROXY_CREDENTIAL_OPTIONS = [
        'CURLOPT_PROXYUSERPWD',
        'CURLOPT_PROXYUSERNAME',
        'CURLOPT_PROXYPASSWORD',
        'CURLOPT_PROXY_SSLCERT',
        'CURLOPT_PROXY_SSLKEY',
        'CURLOPT_PROXY_KEYPASSWD',
        'CURLOPT_PROXY_TLSAUTH_USERNAME',
        'CURLOPT_PROXY_TLSAUTH_PASSWORD',
    ];

    private const PERSISTENT_REQUIRE_FRESH_PROXY_TUNNEL_MESSAGE = 'Persistent cURL sharing is required, but this request requires a fresh proxy tunnel connection.';

    private const CURL_CONNECTION_ERRORS = [
        5 => true,   // CURLE_COULDNT_RESOLVE_PROXY
        6 => true,   // CURLE_COULDNT_RESOLVE_HOST
        7 => true,   // CURLE_COULDNT_CONNECT
        35 => true,  // CURLE_SSL_CONNECT_ERROR
        51 => true,  // CURLE_PEER_FAILED_VERIFICATION before libcurl 7.62.0
        60 => true,  // CURLE_SSL_CACERT / modern CURLE_PEER_FAILED_VERIFICATION
        83 => true,  // CURLE_SSL_ISSUER_ERROR
        90 => true,  // CURLE_SSL_PINNEDPUBKEYNOTMATCH
        91 => true,  // CURLE_SSL_INVALIDCERTSTATUS
        96 => true,  // CURLE_QUIC_CONNECT_ERROR
        97 => true,  // CURLE_PROXY
        98 => true,  // CURLE_SSL_CLIENTCERT
        101 => true, // CURLE_ECH_REQUIRED
    ];

    private const CURL_NETWORK_ERRORS = [
        16 => true, // CURLE_HTTP2
        52 => true, // CURLE_GOT_NOTHING
        55 => true, // CURLE_SEND_ERROR
        56 => true, // CURLE_RECV_ERROR
        92 => true, // CURLE_HTTP2_STREAM
        95 => true, // CURLE_HTTP3
    ];

    private const CURL_RESPONSE_TRANSFER_ERRORS = [
        8 => true,  // CURLE_WEIRD_SERVER_REPLY
        18 => true, // CURLE_PARTIAL_FILE
        23 => true, // CURLE_WRITE_ERROR from libcurl's content decoders
        61 => true, // CURLE_BAD_CONTENT_ENCODING
    ];

    private const CURL_CONNECT_TIMEOUT_ERRORS = [
        'Connection timed out',
        'Connection timeout',
        'Connection time-out',
        'Resolving timed out',
        'name lookup timed out',
        'Proxy CONNECT aborted due to timeout',
        'SSL connection timeout',
    ];

    /**
     * libcurl's CURL_READFUNC_ABORT value.
     */
    private const CURL_READFUNC_ABORT = 0x10000000;

    /**
     * libcurl's CURL_SEEKFUNC_OK value.
     */
    private const CURL_SEEKFUNC_OK = 0;

    /**
     * libcurl's CURL_SEEKFUNC_FAIL value.
     */
    private const CURL_SEEKFUNC_FAIL = 1;

    /**
     * libcurl's CURL_SEEKFUNC_CANTSEEK value.
     */
    private const CURL_SEEKFUNC_CANTSEEK = 2;

    /**
     * Maximum number of times libcurl may replay a streamed request body.
     */
    private const CURL_SEEK_MAX_REPLAYS = 3;

    /**
     * libcurl's CURLE_SEND_FAIL_REWIND value.
     */
    private const CURLE_SEND_FAIL_REWIND = 65;

    /**
     * Guzzle's default connect timeout in milliseconds, replacing libcurl's
     * 300 seconds.
     */
    private const DEFAULT_CONNECT_TIMEOUT_MS = 60000;

    /**
     * Milliseconds passed to libcurl to disable the connect timeout: libcurl
     * treats zero as its own 300-second default, so disabling means passing
     * the largest value a 32-bit libcurl long accepts.
     */
    private const CONNECT_TIMEOUT_DISABLED_MS = 2147483647;

    /**
     * @var resource[]|\CurlHandle[]
     */
    private array $handles = [];

    /**
     * @var string|null Owner signature of the proxy tunnels that pooled idle
     *                  handles may still hold
     */
    private ?string $proxyTunnelOwner = null;

    /**
     * @var bool Whether an in-domain handle has been pooled since the last purge
     */
    private bool $poolMayHoldTunnels = false;

    /**
     * @var int Total number of idle handles to keep in cache
     */
    private int $maxHandles;

    private bool $closed = false;

    /**
     * @var resource|\CurlShareHandle|\CurlSharePersistentHandle|null
     */
    private $shareHandle;

    private string $shareMode;

    /**
     * @var bool Whether the configured share handle may own a connection
     *           cache populated outside this factory
     */
    private bool $opaqueShareConnectionCache = false;

    /**
     * @param int                                                                            $maxHandles  Maximum number of idle handles.
     * @param resource|\CurlShareHandle|\CurlSharePersistentHandle|CurlShareHandleState|null $shareHandle
     */
    public function __construct(int $maxHandles, string $shareMode = TransportSharing::NONE, $shareHandle = null)
    {}

    /**
     * @param mixed $value
     */
    private static function isCurlShareHandle($value): bool
    {}

    public function create(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ): EasyHandle {}

    /**
     * @param resource|\CurlHandle     $handle
     * @param array<int|string, mixed> $conf
     */
    private function applyCurlOptions(
        #[\SensitiveParameter]
        $handle,
        #[\SensitiveParameter]
        array $conf
    ): void {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function normalizeStringableProxyCredentialOptions(
        #[\SensitiveParameter]
        array &$conf
    ): void {}

    private function rejectRequestLevelShareConflict(
        #[\SensitiveParameter]
        array $options
    ): void {}

    private function rejectPersistentRequireConnectionReuseConflicts(
        #[\SensitiveParameter]
        array $options
    ): void {}

    private static function normalizeMultiplex(
        #[\SensitiveParameter]
        array $options
    ): string {}

    private static function assertRequiredMultiplexSupported(
        #[\SensitiveParameter]
        EasyHandle $easy
    ): void {}

    /**
     * libcurl forces NTLM-authenticated transfers onto HTTP/1.1: when the
     * server picks NTLM from the offered mask, the connection is closed and the
     * request is retried over HTTP/1.1 whatever HTTP version was asked for,
     * silently defeating the required protocol guarantee on both cleartext and
     * TLS routes. The final merged mask is checked because the raw
     * CURLOPT_HTTPAUTH cURL option is allow-listed, and any mask permitting
     * NTLM, such as CURLAUTH_ANY, is rejected because the selection is
     * server-controlled.
     *
     * @param array<int|string, mixed> $conf
     */
    private static function assertRequiredMultiplexAuthSupported(
        #[\SensitiveParameter]
        array $conf
    ): void {}

    private static function assertSelectedProxySupported(
        #[\SensitiveParameter]
        ?string $selectedProxy,
        #[\SensitiveParameter]
        RequestInterface $request
    ): void {}

    /**
     * @param int|string $option
     */
    private static function formatCurlOption($option): string
    {}

    private static function rejectConflictingCurlOptions(
        #[\SensitiveParameter]
        array $options
    ): void {}

    private static function rejectUnsupportedCurlOptions(
        #[\SensitiveParameter]
        array $options
    ): void {}

    private static function rejectUnsupportedRequestOptions(
        #[\SensitiveParameter]
        array $options
    ): void {}

    private static function assertOnStatsCallable(
        #[\SensitiveParameter]
        array $options
    ): void {}

    private static function assertOnTrailersCallable(
        #[\SensitiveParameter]
        array $options
    ): void {}

    /**
     * @return array<int, string|null>
     */
    private static function conflictingCurlOptions(): array
    {}

    /**
     * @return array<int, true>
     */
    private static function supportedCurlOptions(): array
    {}

    /**
     * @param array<int, true> $options
     */
    private static function addSupportedCurlOption(array &$options, string $constant): void
    {}

    /**
     * @param array<int, string|null> $options
     */
    private static function addConflictingCurlOption(array &$options, string $constant, ?string $replacement): void
    {}

    private static function validateRequestUriScheme(
        #[\SensitiveParameter]
        RequestInterface $request
    ): void {}

    public function release(
        #[\SensitiveParameter]
        EasyHandle $easy
    ): void {}

    /**
     * Closes idle cURL handles owned by this factory.
     *
     * After closing, the factory is terminal and must not be reused.
     */
    public function close(): void
    {}

    private function assertOpen(): void
    {}

    private function doClose(bool $explicit): void
    {}

    /**
     * @param resource|\CurlHandle $handle
     */
    private function discardHandle(
        #[\SensitiveParameter]
        $handle
    ): void {}

    /**
     * @param resource|\CurlHandle $handle
     */
    private function clearEasyHandleCallbacks(
        #[\SensitiveParameter]
        $handle
    ): void {}

    /**
     * Completes a cURL transaction, either returning a response promise or a
     * rejected promise.
     *
     * @param callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed> $handler
     * @param CurlFactoryInterface                                                                            $factory Dictates how the handle is released
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    public static function finish(
        callable $handler,
        #[\SensitiveParameter]
        EasyHandle $easy,
        CurlFactoryInterface $factory
    ): PromiseInterface {}

    private static function createStats(
        #[\SensitiveParameter]
        EasyHandle $easy
    ): TransferStats {}

    /**
     * @param callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed> $handler
     * @param (callable(TransferStats): mixed)|null                                                           $onStats
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private static function finishError(
        callable $handler,
        #[\SensitiveParameter]
        EasyHandle $easy,
        CurlFactoryInterface $factory,
        ?TransferStats $stats,
        ?callable $onStats
    ): PromiseInterface {}

    private static function shouldFinishWithError(EasyHandle $easy): bool
    {}

    private static function hasLocalFailure(EasyHandle $easy): bool
    {}

    private static function shouldRetryFailedRewind(EasyHandle $easy): bool
    {}

    /**
     * Whether the transfer failed because libcurl could not rewind the
     * request body to resend it in reply to a response, such as a 401 or
     * 407 during multi-pass authentication, or a 417 that makes libcurl
     * retry an Expect: 100-continue upload without the expectation. Rewinds
     * for replays that follow no response, such as a reused connection that
     * died or a refused HTTP/2 stream, retry through
     * shouldRetryFailedRewind() instead. Followed redirects also rewind,
     * but the built-in handlers never enable them.
     */
    private static function isResponseRewindFailure(EasyHandle $easy): bool
    {}

    private static function createErrorContext(EasyHandle $easy): array
    {}

    /**
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private static function createRejection(
        #[\SensitiveParameter]
        EasyHandle $easy,
        #[\SensitiveParameter]
        array $ctx,
        #[\SensitiveParameter]
        ?\Throwable $previous = null
    ): PromiseInterface {}

    /**
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private static function createRequestOrResponseRejection(
        #[\SensitiveParameter]
        EasyHandle $easy,
        string $message,
        #[\SensitiveParameter]
        ?\Throwable $previous = null
    ): PromiseInterface {}

    private static function isConnectionError(int $errno): bool
    {}

    private static function isNetworkError(int $errno): bool
    {}

    private static function isResponseTransferError(int $errno): bool
    {}

    private static function isConnectTimeoutError(string $error): bool
    {}

    private static function sanitizeCurlError(
        #[\SensitiveParameter]
        string $error,
        #[\SensitiveParameter]
        UriInterface $uri,
        #[\SensitiveParameter]
        ?string $proxy = null
    ): string {}

    private static function redactProxyUserInfo(
        #[\SensitiveParameter]
        string $error,
        #[\SensitiveParameter]
        ?string $proxy
    ): string {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private function forceFreshConnectionForAuthenticatedProxy(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array &$conf
    ): void {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private function isOpaqueShareAnonymousProxyTunnel(RequestInterface $request, string $proxy, array $conf): bool
    {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function getEffectiveProxy(array $conf): ?string
    {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function normalizeCurlHeaderOptions(
        #[\SensitiveParameter]
        array &$conf
    ): void {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function requiresFreshConnectionForAuthenticatedProxy(RequestInterface $request, string $proxy, array $conf): bool
    {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function applyProxyConnectHeaderSuppression(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array &$conf
    ): void {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function usesProxyTunnel(RequestInterface $request, array $conf): bool
    {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function hasCurlConnectTo(array $conf): bool
    {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function isHttpProxyForConnectionReuse(string $proxy, array $conf): bool
    {}

    private static function proxyScheme(string $proxy): ?string
    {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function isSocksProxy(string $proxy, array $conf): bool
    {}

    /**
     * Computes the connection-reuse section signature for a SOCKS proxy.
     * libcurl compares SOCKS credentials on connection reuse from 7.69.0 (curl
     * #4835), so no sectioning is needed there. Older libcurl matches a SOCKS
     * proxy by type, host, and port only, so every SOCKS request is sectioned
     * by its credential state; hashing the credential-less state too keeps an
     * unauthenticated request from inheriting an authenticated connection. See
     * docs/contributing/curl-connection-reuse.md.
     *
     * @param array<int|string, mixed> $conf
     */
    private static function socksProxySignature(
        #[\SensitiveParameter]
        string $proxy,
        #[\SensitiveParameter]
        array $conf
    ): ?string {}

    /**
     * @param mixed $proxyType
     */
    private static function isSocksProxyType($proxyType): bool
    {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function hasCurlProxyCredentials(array $conf): bool
    {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function hasCurlProxyTlsCredentials(array $conf): bool
    {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function hasCurlProxyAuthorizationHeader(array $conf): bool
    {}

    /**
     * Collects the first-class Proxy-Authorization request header lines that
     * Guzzle configures in cURL's proxy-only header channel. Empty values use
     * cURL's semicolon form so they suppress an automatically generated proxy
     * authorization field without carrying a credential. getHeaderLine() is
     * deliberately not used: comma-joining multiple credentials would change
     * their wire representation and connection signature.
     *
     * @return list<string>
     */
    private static function managedProxyAuthorizationHeaderLines(
        #[\SensitiveParameter]
        RequestInterface $request
    ): array {}

    /**
     * Routes the managed Proxy-Authorization lines and any caller-provided
     * proxy headers through cURL's proxy-only header channel without
     * consulting Guzzle's route prediction, so no proxy-classification
     * discrepancy can disclose them to an origin server. libcurl uses the
     * proxy-only list only for HTTP requests it actually sends to a proxy, so
     * direct, no-proxy-bypassed, and SOCKS transfers never deliver it.
     *
     * @param array<int|string, mixed> $conf
     * @param list<string>             $managedHeaders
     */
    private static function applyProxyAuthorizationHeaderHandling(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array &$conf,
        #[\SensitiveParameter]
        array $managedHeaders
    ): void {}

    /**
     * @param array<int|string, mixed> $conf
     * @param list<string>             $headers
     */
    private static function appendCurlProxyHeaders(
        #[\SensitiveParameter]
        array &$conf,
        #[\SensitiveParameter]
        array $headers
    ): void {}

    /**
     * @param array<int|string, mixed> $conf
     */
    private static function hasCurlProxyHeaderOption(array $conf): bool
    {}

    /**
     * @param mixed[] $headers
     *
     * @return list<string>
     */
    private static function proxyAuthorizationHeaderValuesFromList(array $headers): array
    {}

    /**
     * Computes the connection-reuse section signature for a proxy tunnel or
     * SOCKS proxy, or null when the request does not require sectioning.
     *
     * @param array<int|string, mixed> $conf
     */
    private static function proxyTunnelSignature(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $conf
    ): ?string {}

    /**
     * @param array<int|string, mixed> $conf
     *
     * @return list<string>
     */
    private static function curlProxyAuthorizationHeaderValues(array $conf): array
    {}

    private function discardIdleHandles(): void
    {}

    /**
     * @return array<int|string, mixed>
     */
    private function getDefaultConf(
        #[\SensitiveParameter]
        EasyHandle $easy
    ): array {}

    /**
     * @param string[] $protocols
     */
    private static function curlProtocolMask(array $protocols): int
    {}

    /**
     * @param mixed $type
     */
    private static function normalizeTlsFileType(string $option, $type): string
    {}

    private static function shouldValidateSslKeyFile(?string $type): bool
    {}

    private function applyMethod(
        #[\SensitiveParameter]
        EasyHandle $easy,
        #[\SensitiveParameter]
        array &$conf,
        RequestFraming $framing
    ): void {}

    private function applyBody(
        #[\SensitiveParameter]
        EasyHandle $easy,
        #[\SensitiveParameter]
        array &$conf,
        RequestFraming $framing
    ): void {}

    private static function curlInputSizeOption(
        #[\SensitiveParameter]
        RequestInterface $request,
        int $contentLength
    ): int {}

    private function applyHeaders(
        #[\SensitiveParameter]
        EasyHandle $easy,
        #[\SensitiveParameter]
        array &$conf
    ): void {}

    /**
     * Remove a header from the options array.
     *
     * @param string $name    Case-insensitive header to remove
     * @param array  $options Array of options to modify
     */
    private function removeHeader(string $name, array &$options): void
    {}

    /**
     * Decorates a caller-owned sink stream so that closing the response body
     * detaches Guzzle's wrapper without closing the original PHP resource.
     */
    private static function streamForResourceSink(StreamInterface $stream): StreamInterface
    {}

    /**
     * @param mixed $factory
     */
    private static function requireStreamFactory($factory): StreamFactoryInterface
    {}

    private function applyHandlerOptions(
        #[\SensitiveParameter]
        EasyHandle $easy,
        #[\SensitiveParameter]
        array &$conf
    ): void {}

    private function applyTlsVersionRange(
        #[\SensitiveParameter]
        EasyHandle $easy,
        #[\SensitiveParameter]
        array &$conf
    ): void {}

    private static function curlMinSslVersion(
        #[\SensitiveParameter]
        EasyHandle $easy,
        int $value
    ): int {}

    private static function curlMaxSslVersion(
        #[\SensitiveParameter]
        EasyHandle $easy,
        int $value
    ): int {}

    private static function requireCurlMaxSslVersion(
        #[\SensitiveParameter]
        EasyHandle $easy,
        string $constant
    ): int {}

    /**
     * This function ensures that a response was set on a transaction. If one
     * was not set, then the request is retried if possible. This error
     * typically means you are sending a payload, curl encountered a
     * "Connection died, retrying a fresh connect" error, tried to rewind the
     * stream, and then encountered a "necessary data rewind wasn't possible"
     * error, causing the request to be sent through curl_multi_info_read()
     * without an error status.
     *
     * @param callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed> $handler
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private static function retryFailedRewind(
        callable $handler,
        #[\SensitiveParameter]
        EasyHandle $easy,
        #[\SensitiveParameter]
        array $ctx
    ): PromiseInterface {}

    /**
     * Parses validated trailer field lines into an associative array keyed by
     * lowercased field name, preserving first-occurrence key order and wire
     * value order.
     *
     * @param list<string> $lines
     *
     * @return array<string, list<string>>
     */
    private static function headersFromTrailerLines(array $lines): array
    {}

    private function createHeaderFn(
        #[\SensitiveParameter]
        EasyHandle $easy
    ): callable {}

    public function __destruct()
    {}

    public function __unserialize(array $data): void
    {}
}
