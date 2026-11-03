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
use GuzzleHttp\Exception\TransferException;
use GuzzleHttp\HostIdentity;
use GuzzleHttp\Multiplexing;
use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\Promise as P;
use GuzzleHttp\Promise\PromiseInterface;
use GuzzleHttp\ProxyOptions;
use GuzzleHttp\Psr7;
use GuzzleHttp\Psr7\Exception\TimeoutException;
use GuzzleHttp\RequestOptions;
use GuzzleHttp\TransferStats;
use GuzzleHttp\TransportSharing;
use GuzzleHttp\Utils;
use Openssl\Session;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseFactoryInterface;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Message\StreamFactoryInterface;
use Psr\Http\Message\StreamInterface;
use Psr\Http\Message\UriInterface;

/**
 * HTTP handler that uses PHP's HTTP stream wrapper.
 */
final class StreamHandler
{
    use NonSerializableTrait;

    private const KNOWN_CONSTRUCTOR_OPTIONS = [
        'max_host_connections' => true,
        'max_total_connections' => true,
        'transport_sharing' => true,
    ];

    /**
     * Peers tracked by the TLS session cache, matching libcurl's share handle
     * TLS session cache (25 peers, up to 2 sessions each).
     */
    private const TLS_SESSION_CACHE_MAX_KEYS = 25;

    private const CONNECTION_ERRORS = [
        'php_network_getaddresses:',
        'getaddrinfo',
        'gethostbyname failed',
        'Unable to connect to',
        'Connection refused',
        'No connection could be made because the target machine actively refused it',
        'connection attempt failed',
        'connect() failed',
        'Network is unreachable',
        'No route to host',
        'Host is unreachable',
        'Host is down',
        'Cannot connect to HTTPS server through proxy',
        'Failed to enable crypto',
    ];

    private const CONNECT_TIMEOUT_ERRORS = [
        'Connection timed out',
        'Operation timed out',
        'SSL: Handshake timed out',
        'did not properly respond after a period of time',
    ];

    private const NETWORK_ERRORS = [
        'SSL: Connection reset by peer',
        'SSL: Broken pipe',
        'unexpected eof while reading',
    ];

    // PHP's HTTP wrapper can collapse wrapper-level errors to OpenFailed.
    // These codes classify only errors delivered directly to the context
    // handler. NetworkRecvFailed is omitted because PHP currently emits it for
    // handshake-time certificate policy failures.
    private const STRUCTURED_NETWORK_ERROR_CODES = [
        'NetworkSendFailed',
    ];

    // SslNotSupported can be emitted directly by a transport that cannot
    // enable crypto. Standard ext-openssl proxy failures are normally
    // collapsed to OpenFailed and continue to use message classification.
    private const STRUCTURED_CONNECTION_ERROR_CODES = [
        'SslNotSupported',
    ];

    /**
     * Default idle timeout in milliseconds when the "read_timeout" option is
     * not set. Matches PHP's default_socket_timeout default, which the
     * handler never consults.
     */
    private const DEFAULT_IDLE_TIMEOUT_MS = 60000;

    private ?\Throwable $onStatsException = null;

    private string $transportSharingMode;

    private bool $connectionCapsConfigured = false;

    private ?StreamTlsSessionCache $sessionCache = null;

    /**
     * Accepts an associative array of options:
     *
     * - max_host_connections: Optional positive integer or null. A non-null
     *   value marks the handler as incompatible with enabled response
     *   streaming; the number is not used for stream-handler admission.
     * - max_total_connections: Optional positive integer or null. A non-null
     *   value marks the handler as incompatible with enabled response
     *   streaming; the number is not used for stream-handler admission.
     * - transport_sharing: Optional transport sharing mode.
     *
     * The stream handler cannot cap streamed connections, so a configured cap
     * marker rejects enabled response streaming ("stream" => true). Accepted
     * transfers are buffered and hold at most one connection per in-flight
     * call, but overlapping buffered calls are not collectively limited.
     *
     * @param array{max_host_connections?: mixed, max_total_connections?: mixed, transport_sharing?: mixed} $options Array of options to use with the handler
     */
    public function __construct(array $options = [])
    {}

    /**
     * Sends an HTTP request.
     *
     * @param RequestInterface $request Request to send.
     * @param array            $options Request transfer options.
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    public function __invoke(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ): PromiseInterface {}

    private function isOnStatsException(\Throwable $e): bool
    {}

    private static function isConnectTimeoutError(string $message): bool
    {}

    private static function isConnectionError(string $message): bool
    {}

    private static function isSendError(string $message): bool
    {}

    private static function isNetworkError(string $message): bool
    {}

    /**
     * @param string[] $streamErrorCodes
     */
    private static function createStreamFailureException(
        #[\SensitiveParameter]
        string $message,
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        \Exception $previous,
        array $streamErrorCodes
    ): TransferException {}

    /**
     * @param string[] $streamErrorCodes
     * @param string[] $codes
     */
    private static function hasStreamErrorCode(array $streamErrorCodes, array $codes): bool
    {}

    private function invokeStats(
        #[\SensitiveParameter]
        array $options,
        #[\SensitiveParameter]
        RequestInterface $request,
        ?float $startTime,
        #[\SensitiveParameter]
        ?ResponseInterface $response = null,
        #[\SensitiveParameter]
        ?\Throwable $error = null
    ): void {}

    /**
     * @param resource $stream
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private function createResponse(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options,
        $stream,
        #[\SensitiveParameter]
        array $hdrs,
        ?float $startTime,
        ?float $deadline
    ): PromiseInterface {}

    /**
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private function rejectResponseCreation(
        #[\SensitiveParameter]
        array $options,
        #[\SensitiveParameter]
        RequestInterface $request,
        ?float $startTime,
        #[\SensitiveParameter]
        \Throwable $previous
    ): PromiseInterface {}

    private function createSink(
        StreamInterface $stream,
        #[\SensitiveParameter]
        array $options
    ): StreamInterface {}

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

    /**
     * @param mixed $factory
     */
    private static function requireResponseFactory($factory): ResponseFactoryInterface
    {}

    /**
     * Removes Transfer-Encoding values that start with chunked from a response
     * that cannot have a body, as PHP's HTTP wrapper did.
     *
     * @param array<string, string[]> $headers
     *
     * @return array<string, string[]>
     */
    private static function removeChunkedTransferCoding(array $headers): array
    {}

    /**
     * @return array{0: StreamInterface, 1: array, 2: ?EncodedBodyStream}
     */
    private static function checkDecode(
        #[\SensitiveParameter]
        array $options,
        array $headers,
        StreamInterface $stream,
        ?string $declaredLength
    ): array {}

    /**
     * Drains the source stream into the "sink" client option.
     *
     * @throws \RuntimeException when the sink option is invalid.
     */
    private function drain(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        ResponseInterface $response,
        StreamInterface $source,
        StreamInterface $sink,
        ?string $declaredResponseBodyLength,
        ?EncodedBodyStream $encodedBody = null
    ): StreamInterface {}

    private function createResponseSink(
        RequestInterface $request,
        ResponseInterface $response,
        StreamInterface $sink
    ): StreamInterface {}

    /**
     * Decorates the transport stream so each buffered-body read observes the
     * remaining wall-clock budget of the "timeout" request option, bounded by
     * the "read_timeout" idle timeout when that is shorter. The transport is
     * switched to non-blocking mode because a blocking read waits for data up
     * to the transport's own timeout, which would keep the deadline from being
     * enforced while the server stalls.
     *
     * @param resource $resource
     */
    private static function createDeadlineSource(
        StreamInterface $stream,
        $resource,
        float $deadline,
        #[\SensitiveParameter]
        array $options
    ): StreamInterface {}

    /**
     * Create a resource and check to ensure it was created successfully
     *
     * @param callable(): (resource|false) $callback Callable that returns a stream resource, or false when resource creation fails.
     *
     * @return resource
     *
     * @throws \RuntimeException when the callback returns false or resource creation emits an error.
     */
    private function createResource(
        callable $callback,
        #[\SensitiveParameter]
        ?UriInterface $diagnosticUri = null
    ) {}

    /**
     * @return resource
     */
    private function createStream(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options,
        string $body,
        ?float &$deadline
    ) {}

    /**
     * @param string[] $streamErrorCodes
     */
    private function addStructuredStreamErrorHandler(
        #[\SensitiveParameter]
        array &$context,
        array &$streamErrorCodes,
        bool &$captureStreamErrors
    ): void {}

    private static function supportsStructuredStreamErrors(): bool
    {}

    private static function assertRequestUriSupported(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ): void {}

    private function applyHandlerOptions(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array &$context,
        #[\SensitiveParameter]
        array $options,
        array &$params
    ): void {}

    private function resolveHost(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ): UriInterface {}

    /**
     * Returns a numeric IPv4 spelling folded to the dotted quad libcurl's
     * ipv4_normalize() produces, and every other host unchanged.
     */
    private static function canonicalConnectionHost(string $host): string
    {}

    private function addDefaultTlsMinimum(RequestInterface $request, array &$context): void
    {}

    /**
     * Wires PHP 8.6+ TLS session resumption into the SSL context. Preferred
     * sharing modes fall back to no sharing when a request cannot safely use
     * Guzzle-managed sessions; HANDLER_REQUIRE fails loudly instead.
     *
     * New sessions are held temporarily and committed only after the HTTPS
     * stream opens successfully. If PHP rejects peer verification or another
     * stream-open step fails, any session reported during that attempt is
     * discarded.
     *
     * @param array $customSslContext User-supplied stream_context['ssl']
     *                                values.
     */
    private function applyTlsSessionResumption(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array &$context,
        #[\SensitiveParameter]
        array $customSslContext = []
    ): ?\Closure {}

    private function transportSharingRequested(): bool
    {}

    private function transportSharingRequired(): bool
    {}

    private function failRequiredTlsSharingForRequest(
        #[\SensitiveParameter]
        RequestInterface $request,
        string $reason
    ): void {}

    private function failRequiredTlsSharingForConfiguration(string $reason): void
    {}

    /**
     * Returns this handler's TLS session cache, or null when the configured
     * sharing mode shares nothing or the OpenSSL session API is unavailable.
     * Persistent (process-wide) sharing is a cURL-only feature, so
     * PERSISTENT_PREFER degrades to this per-handler cache while
     * PERSISTENT_REQUIRE is rejected before sharing applies.
     */
    private function sessionCache(): ?StreamTlsSessionCache
    {}

    private function getDefaultContext(
        #[\SensitiveParameter]
        RequestInterface $request,
        string $body
    ): array {}

    private function assertTransportSharingSupported(): void
    {}

    private static function rejectUnsupportedRequestOptions(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ): void {}

    private function rejectStreamingWithConnectionCaps(
        #[\SensitiveParameter]
        array $options
    ): void {}

    private static function rejectConflictingStreamContextOptions(
        #[\SensitiveParameter]
        array $streamContext
    ): void {}

    private static function rejectUnsupportedStreamContextOptions(
        #[\SensitiveParameter]
        array $streamContext
    ): void {}

    /**
     * @return string[]
     */
    private static function unsupportedStreamContextOptions(array $streamContext): array
    {}

    /**
     * @return array<string, array<string, true>>
     */
    private static function supportedStreamContextOptions(): array
    {}

    /**
     * @return array<string, array<string, string>>
     */
    private static function conflictingStreamContextOptions(): array
    {}

    /**
     * @param mixed $value as passed via Request transfer options.
     *
     * @return array{0: string, 1: string|null}
     */
    private static function normalizeTlsFileOption(
        string $option,
        #[\SensitiveParameter]
        $value
    ): array {}

    private static function setTlsPassphrase(
        #[\SensitiveParameter]
        array &$options,
        #[\SensitiveParameter]
        ?string $passphrase,
        string $option
    ): void {}

    /**
     * @param mixed $value as passed via Request transfer options.
     */
    private static function assertStreamTlsType(string $option, $value): void
    {}

    /**
     * @param mixed $value as passed via Request transfer options.
     */
    private function applyProxy(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array &$context,
        #[\SensitiveParameter]
        $value
    ): void {}

    /**
     * Whether the scheme is a TCP-family transport name that can carry HTTP
     * for a proxy (tcp, ssl, tls, tlsv1.*), regardless of build support.
     */
    private static function isRawTransportName(string $scheme): bool
    {}

    /**
     * Parses the given proxy URL to make it compatible with the format PHP's
     * stream context expects.
     */
    private function parseProxy(string $url, string $scheme): array
    {}

    /**
     * @param mixed $value as passed via Request transfer options.
     */
    private function applyCryptoMethodOption(
        #[\SensitiveParameter]
        array &$context,
        $value
    ): void {}

    /**
     * @param mixed $value as passed via Request transfer options.
     */
    private function applyCryptoMethodMaxOption(
        #[\SensitiveParameter]
        array &$context,
        $value
    ): void {}

    private static function assertTlsVersionRangeForOptions(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ): void {}

    /**
     * @param mixed $value as passed via Request transfer options.
     */
    private function applyVerifyOption(
        #[\SensitiveParameter]
        array &$context,
        $value
    ): void {}

    /**
     * @param mixed $value as passed via Request transfer options.
     */
    private function applyCertOption(
        #[\SensitiveParameter]
        array &$context,
        #[\SensitiveParameter]
        $value
    ): void {}

    /**
     * @param mixed $value as passed via Request transfer options.
     */
    private function applyCertTypeOption($value): void
    {}

    /**
     * @param mixed $value as passed via Request transfer options.
     */
    private function applySslKeyOption(
        #[\SensitiveParameter]
        array &$context,
        #[\SensitiveParameter]
        $value
    ): void {}

    /**
     * @param mixed $value as passed via Request transfer options.
     */
    private function applySslKeyTypeOption($value): void
    {}

    /**
     * @param mixed $value as passed via Request transfer options.
     */
    private function applyProgressOption($value, array &$params): void
    {}

    /**
     * @param mixed $value as passed via Request transfer options.
     */
    private function applyDebugOption(
        #[\SensitiveParameter]
        RequestInterface $request,
        $value,
        array &$params
    ): void {}

    private static function addNotification(array &$params, callable $notify): void
    {}

    private static function callArray(array $functions): callable
    {}
}
