<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use Closure;
use GuzzleHttp\Exception\HandlerClosedException;
use GuzzleHttp\Exception\InvalidArgumentException;
use GuzzleHttp\Exception\RequestException;
use GuzzleHttp\Exception\ResponseException;
use GuzzleHttp\Multiplexing;
use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\Promise as P;
use GuzzleHttp\Promise\Promise;
use GuzzleHttp\Promise\PromiseInterface;
use GuzzleHttp\Psr7;
use GuzzleHttp\TransportSharing;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;

/**
 * Returns an asynchronous response using curl_multi_* functions.
 *
 * When using the CurlMultiHandler, custom curl options can be specified as an
 * associative array of curl option constants mapping to values in the
 * **curl** key of the provided request options.
 */
final class CurlMultiHandler
{
    use NonSerializableTrait;

    private const KNOWN_CONSTRUCTOR_OPTIONS = [
        'handle_factory' => true,
        'max_host_connections' => true,
        'max_total_connections' => true,
        'multiplex' => true,
        'options' => true,
        'select_timeout' => true,
        'transport_sharing' => true,
    ];

    private const CONNECTION_CAP_OPTIONS = [
        'max_host_connections' => 'CURLMOPT_MAX_HOST_CONNECTIONS',
        'max_total_connections' => 'CURLMOPT_MAX_TOTAL_CONNECTIONS',
    ];

    /**
     * cURL options that isolate a transfer from foreign proxy tunnel
     * connections. Failing to apply either one would fall open into
     * credential-bearing connection reuse.
     */
    private const PROXY_TUNNEL_ISOLATION_OPTIONS = [
        'CURLOPT_FRESH_CONNECT',
        'CURLOPT_FORBID_REUSE',
    ];

    private CurlFactoryInterface $factory;

    private bool $ownsFactory;

    private ?CurlShareHandleState $shareHandleState;

    private float $selectTimeout;

    /**
     * @var int Will be higher than 0 when `curl_multi_exec` is still running.
     */
    private int $active = 0;

    /**
     * @var array Request entry handles, indexed by handle id in `addRequest`.
     *
     * @see CurlMultiHandler::addRequest
     */
    private array $handles = [];

    /**
     * @var array<int, float> An array of delay times, indexed by handle id in `addRequest`.
     *
     * @see CurlMultiHandler::addRequest
     */
    private array $delays = [];

    /**
     * @var array<mixed> An associative array of CURLMOPT_* options and corresponding values for curl_multi_setopt()
     */
    private array $options = [];

    /**
     * @var bool Whether the "multiplex" constructor option disabled
     *           multiplexing on this handler's multi handle
     */
    private bool $multiplexDisabled = false;

    /**
     * @var resource|\CurlMultiHandle|null
     */
    private $multiHandle;

    private bool $closed = false;

    private bool $closing = false;

    /**
     * @var int Depth of nested guarded native operations (execution and
     *          handle removal, both of which can run user callbacks). A
     *          callback can re-enter tick(), and the nested frame must not
     *          clear the outer frame's guard; deferred work stays parked
     *          until the outermost frame unwinds.
     */
    private int $multiExecDepth = 0;

    /**
     * @var bool Guards finishDeferredWork() against re-entry from the
     *           guarded native removals it performs while flushing.
     */
    private bool $finishingDeferredWork = false;

    /**
     * @var array<int, array{easy: EasyHandle, attached: bool}>
     */
    private array $deferredCancels = [];

    /**
     * @var array<int, object|null> Wait tokens of requests created from inside
     *                              a cURL callback, keyed by handle id; native
     *                              attachment is deferred until the outermost
     *                              native execution unwinds.
     */
    private array $deferredAdds = [];

    private bool $deferredClose = false;

    private bool $deferredCloseExplicit = false;

    /**
     * @var string|null Owner signature of the proxy tunnels the multi handle's
     *                  connection cache may hold
     */
    private ?string $proxyTunnelOwner = null;

    /** @var array<string, int> Count of attached transfers per proxy tunnel signature. */
    private array $activeProxyTunnelSignatures = [];

    /** @var array<int, string> Maps an attached handle id to its proxy tunnel signature. */
    private array $activeProxyTunnelHandles = [];

    /**
     * @var int Depth of nested processMessages() calls. Guards against
     *          multi-handle recreation re-entrancy from processMessages (a
     *          retried transfer re-invokes the handler) and keeps deferred
     *          work parked until the outermost message loop unwinds.
     */
    private int $messageProcessingDepth = 0;

    /**
     * This handler accepts the following options:
     *
     * - handle_factory: An optional factory  used to create curl handles
     * - transport_sharing: Optional transport sharing mode.
     * - select_timeout: Optional timeout (in seconds) to block before timing
     *   out while selecting curl handles. Defaults to 1 second.
     * - max_host_connections: Optional maximum concurrent connections per host.
     * - max_total_connections: Optional maximum concurrent connections overall.
     * - multiplex: Optional Multiplexing::NONE to disallow multiplexing on
     *   this handler's multi handle. The eager, wait, and required modes are
     *   request options, not handler options; Multiplexing::NONE is also
     *   conditionally accepted as a request option value.
     * - options: An associative array of CURLMOPT_* options and
     *   corresponding values for curl_multi_setopt()
     */
    public function __construct(array $options = [])
    {}

    public function __destruct()
    {}

    public function __unserialize(array $data): void
    {}

    /**
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    public function __invoke(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ): PromiseInterface {}

    /**
     * The "multiplex" request option sets CURLOPT_PIPEWAIT, which libcurl
     * ignores entirely when the multi handle disallows multiplexing, so an
     * explicit request for multiplexing on a handler configured with
     * Multiplexing::NONE is a configuration error. The required family
     * conflicts marker-independently: a required guarantee on such a handler
     * is contradictory even when the transfer would not wait. The default
     * WAIT mode adapts instead of conflicting: the handler option wins and
     * nothing waits.
     */
    private function rejectDisabledMultiplexConflict(
        #[\SensitiveParameter]
        EasyHandle $easy,
        #[\SensitiveParameter]
        array $options
    ): void {}

    /**
     * A Multiplexing::NONE request option is a sole-use guarantee: the
     * transfer must not share its connection with any concurrent transfer.
     * It holds structurally on a handler whose "multiplex" option is
     * Multiplexing::NONE, and for HTTP/1.x transfers, which never join a
     * multiplexed connection and open connections nothing can join. An
     * HTTP/2 or HTTP/3 request on a handler that multiplexes is rejected,
     * as is any configuration under which the guarantee cannot be verified
     * (custom handle factories control the native handle), cannot be
     * hardened (challenge-response authentication retries and Expect
     * 417 retries re-enter connection selection as internal follows, which
     * disarm CURLOPT_FRESH_CONNECT), or must not be hardened (required
     * persistent sharing forbids fresh connections). On runtimes whose
     * matcher can hand an HTTP/1.x transfer an idle multiplexed connection
     * (below libcurl 7.77.0, and 8.11.0-8.12.1), accepted transfers force a
     * fresh connection.
     */
    private function applyMultiplexNone(
        #[\SensitiveParameter]
        EasyHandle $easy,
        #[\SensitiveParameter]
        array $options
    ): void {}

    /**
     * @param array<mixed> $options
     */
    private static function rejectConflictingCurlMultiOptions(array $options): void
    {}

    /**
     * @param array<string, mixed> $options
     */
    private static function firstConnectionCapOption(array $options): ?string
    {}

    /**
     * @param array<string, mixed> $options
     */
    private function addConnectionCapOptions(array $options): void
    {}

    /**
     * @param int|string $option
     */
    private static function formatCurlMultiOption($option): string
    {}

    /**
     * @return array<int, string>
     */
    private static function conflictingCurlMultiOptions(): array
    {}

    /**
     * @param array<int, string> $options
     */
    private static function addConflictingCurlMultiOption(array &$options, string $constant, string $replacement): void
    {}

    /**
     * Isolates the connection cache when the request's proxy tunnel section
     * differs from the one the multi handle's cache may already hold.
     */
    private function applyProxyTunnelOwnership(
        #[\SensitiveParameter]
        EasyHandle $easy
    ): void {}

    private function addHandleToMulti(
        int $id,
        #[\SensitiveParameter]
        EasyHandle $easy
    ): void {}

    private function isolateFromForeignActiveProxyTunnel(
        #[\SensitiveParameter]
        EasyHandle $easy
    ): void {}

    private function isolateProxyTunnelTransfer(
        #[\SensitiveParameter]
        EasyHandle $easy
    ): void {}

    private static function proxyTunnelIsolationFailureMessage(string $name): string
    {}

    private function markProxyTunnelActive(int $id, EasyHandle $easy): void
    {}

    private function unmarkProxyTunnelActiveById(int $id): void
    {}

    /**
     * Ticks the curl event loop.
     */
    public function tick(): void
    {}

    /**
     * Ticks the curl event loop, returning before the blocking select if the
     * targeted transfer has settled, been canceled, or been replaced by a
     * request that reused its native handle ID.
     */
    private function tickFor(?int $targetId, ?object $waitToken): void
    {}

    /**
     * Runs \curl_multi_exec() inside the event loop, to prevent busy looping
     */
    private function tickInQueue(): void
    {}

    /**
     * Runs until all outstanding connections have completed.
     */
    public function execute(): void
    {}

    /**
     * Runs the event loop until the given transfer has finished, so waiting
     * on a promise does not wait for every other transfer on the handler
     * like execute() does.
     *
     * The native cURL handle ID can be reused by a request created from a
     * completion callback, so the wait token guards against waiting on an
     * unrelated transfer that inherited the ID.
     *
     * @return string Why waiting stopped, for the caller to report when the
     *                transfer did not settle
     */
    private function executeUntil(int $id, object $waitToken): string
    {}

    /**
     * Describes the state that stopped a wait, for the fallback rejection in
     * __invoke(). Only meaningful when the transfer did not settle.
     */
    private function describeStalledWait(int $id, object $waitToken): string
    {}

    /**
     * Checks that the request with the given handle ID is still pending and,
     * when a wait token is given, has not been replaced by a request that
     * reused the ID.
     */
    private function hasRequest(int $id, ?object $waitToken = null): bool
    {}

    /**
     * Closes native cURL resources owned by this handler.
     *
     * Pending transfers are rejected with HandlerClosedException. After
     * closing, the handler is terminal and must not be reused.
     */
    public function close(): void
    {}

    private function assertOpen(): void
    {}

    private function doClose(bool $explicit): void
    {}

    private function deferClose(bool $explicit, ?\Throwable &$failure): void
    {}

    private function finishClose(): void
    {}

    private function captureFailure(?\Throwable &$failure, callable $callback): void
    {}

    /**
     * @param array{easy: EasyHandle, deferred: Promise<ResponseInterface, mixed>, wait_token: object} $entry
     */
    private function discardPendingRequest(int $id, array $entry, \Throwable $failure): \Throwable
    {}

    private function cleanupPendingTransfers(bool $reject, ?\Throwable &$failure): void
    {}

    private function closeMultiHandle(?\Throwable &$failure): void
    {}

    private function closeOwnedFactory(?\Throwable &$failure): void
    {}

    /**
     * @phpstan-impure
     */
    private function executeMulti(): int
    {}

    /**
     * Flushes attachments deferred while the multi handle was busy executing
     * transfers or removing a handle, and cancels and a close deferred while
     * it was executing transfers or processing completion messages.
     */
    private function finishDeferredWork(): void
    {}

    /**
     * Attaches requests whose native attachment was deferred because they
     * were created from inside a cURL callback.
     */
    private function flushDeferredAdds(): void
    {}

    /**
     * Fails a synchronous wait attempted from inside a cURL callback, where
     * native execution cannot progress until the callback returns.
     *
     * @return string Why waiting stopped, for the caller to report when this
     *                method left the transfer unsettled
     */
    private function failNestedWait(int $id, object $token): string
    {}

    private function disposeEasyHandle(
        #[\SensitiveParameter]
        EasyHandle $easy
    ): void {}

    /**
     * @param resource|\CurlHandle $handle
     */
    private function clearEasyHandleCallbacks(
        #[\SensitiveParameter]
        $handle
    ): void {}

    /**
     * @param resource|\CurlHandle $handle
     */
    private function removeHandleFromMulti(
        int $id,
        #[\SensitiveParameter]
        $handle
    ): void {}

    private function hasMultiHandle(): bool
    {}

    private static function hasEasyHandle(EasyHandle $easy): bool
    {}

    private function addRequest(
        #[\SensitiveParameter]
        array $entry
    ): void {}

    /**
     * Cancels a handle from sending and removes references to it.
     *
     * @param int         $id        Handle ID to cancel and remove.
     * @param object|null $waitToken Identity token that must still match the
     *                               entry when given.
     *
     * @return bool True on success, false on failure.
     */
    private function cancel(int $id, ?object $waitToken = null): bool
    {}

    private function cleanupDeferredCancels(?\Throwable &$failure): void
    {}

    private function processMessages(): void
    {}

    /**
     * Bounds a blocking select by the earliest pending request delay so a
     * delayed transfer becoming due does not wait out an unrelated
     * transfer's full select timeout.
     */
    private function effectiveSelectTimeout(): float
    {}

    /**
     * @return float Seconds until the earliest pending delay is due
     */
    private function secondsToNext(): float
    {}

    private function timeToNext(): int
    {}

    /**
     * @return resource|\CurlMultiHandle
     */
    private function getMultiHandle()
    {}
}
