<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\Exception\InvalidArgumentException;
use GuzzleHttp\HostIdentity;
use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\Psr7;
use Openssl\Session;

/**
 * Per-handler in-memory TLS session resumption cache for the stream handler.
 *
 * Backed by the OpenSSL TLS session API added in PHP 8.6. A session is only
 * ever replayed for an identical TLS identity: the non-secret identity is
 * encoded in the lookup key and secret material is matched in constant time.
 * TLS resumption reuses the original certificate-chain verification result
 * instead of building and verifying the chain again. Guzzle includes ambient
 * trust configuration paths in the cache identity, but cannot detect
 * trust-store contents rewritten at an unchanged path. Recreate the handler
 * or disable transport sharing when trust changes must take effect
 * immediately.
 *
 * New sessions are held temporarily and committed only after the HTTPS stream
 * opens successfully. If PHP rejects peer verification or another stream-open
 * step fails, any session reported during that attempt is discarded.
 *
 * @internal
 */
final class StreamTlsSessionCache
{
    use NonSerializableTrait;

    /**
     * Sessions retained per peer. libcurl uses 2 to buffer single-use TLS 1.3
     * tickets; the synchronous stream handler does not need more.
     */
    public const MAX_SESSIONS_PER_KEY = 2;

    /**
     * TLS 1.3 tickets should not live longer than RFC 8446 allows. All
     * pre-TLS-1.3 sessions use libcurl's tighter one-day cap.
     */
    private const MAX_TLS13_LIFETIME = 604800;
    private const MAX_PRE_TLS13_LIFETIME = 86400;

    private const USER_MANAGED_SESSION_OPTIONS = [
        'session_cache' => true,
        'session_cache_size' => true,
        'session_data' => true,
        'session_get_cb' => true,
        'session_id_context' => true,
        'session_new_cb' => true,
        'session_remove_cb' => true,
        'session_stream' => true,
        'session_timeout' => true,
    ];

    private const USER_MANAGED_PSK_OPTIONS = [
        'psk_client_cb' => true,
        'psk_server_cb' => true,
    ];

    /**
     * TLS 1.3 early data (0-RTT) options are deliberately never shared: an
     * injected cached session would let PHP send the replayable early data
     * payload ahead of the HTTP request.
     */
    private const USER_MANAGED_EARLY_DATA_OPTIONS = [
        'early_data' => true,
        'early_data_cb' => true,
        'max_early_data' => true,
    ];

    /**
     * File/path-bearing SSL context options are deliberately not shared: their
     * contents can change outside this handler.
     */
    private const PATH_OPTIONS = [
        'SNI_server_certs' => true,
        'cafile' => true,
        'capath' => true,
        'dh_param' => true,
        'local_cert' => true,
        'local_pk' => true,
    ];

    private const CERT_CAPTURE_OPTIONS = [
        'capture_peer_cert' => true,
        'capture_peer_cert_chain' => true,
    ];

    /**
     * Scalar custom SSL context options that are known to be self-contained
     * and safe to include in the TLS session cache identity.
     */
    private const CUSTOM_SCALAR_KEYABLE_OPTIONS = [
        'SNI_enabled' => true,
        'allow_self_signed' => true,
        'alpn_protocols' => true,
        'ciphers' => true,
        'crypto_method' => true,
        'disable_compression' => true,
        'max_proto_version' => true,
        'min_proto_version' => true,
        'peer_name' => true,
        'security_level' => true,
        'verify_depth' => true,
        'verify_peer' => true,
        'verify_peer_name' => true,
    ];

    private const INVALID_PEER_FINGERPRINT_REASON = 'the SSL context option "peer_fingerprint" must be a string or a non-empty, flat array with string algorithm names and string fingerprints.';

    private int $maxKeys;

    /**
     * @var array<string, list<array{session: Session, credentials: string, expiresAt: int, singleUse: bool}>>
     */
    private array $sessions = [];

    public function __construct(int $maxKeys)
    {}

    public static function isSupported(): bool
    {}

    /**
     * @param array $ssl       The assembled 'ssl' stream context array.
     * @param array $customSsl The user-supplied stream_context['ssl'] array.
     */
    public static function unsupportedContextReason(array $ssl, array $customSsl = []): ?string
    {}

    /**
     * Builds the non-secret lookup key: host/port plus every TLS parameter that
     * defines the verification/identity context. Keyed on the connection host
     * a transport reads, with numeric IPv4 spellings folded to one dotted
     * quad; names are never resolved. Secret material is excluded.
     *
     * @param array $ssl The assembled 'ssl' stream context array.
     */
    public static function peerKey(
        string $host,
        ?int $port,
        #[\SensitiveParameter]
        array $ssl
    ): string {}

    /**
     * Builds the secret-aware credential fingerprint matched in constant time,
     * keeping the passphrase out of the loggable peer key.
     *
     * Every credential-bearing context is currently rejected by
     * unsupportedContextReason() before sharing, so fingerprints can only
     * diverge if the allow-lists are ever widened; the constant-time match is
     * retained as defense in depth for that case.
     *
     * @param array $ssl The assembled 'ssl' stream context array.
     */
    public static function credentialFingerprint(array $ssl): string
    {}

    public function find(string $key, string $credentials): ?Session
    {}

    public function store(
        string $key,
        string $credentials,
        #[\SensitiveParameter]
        Session $session
    ): void {}

    private function touch(string $key): void
    {}

    private function evictExcessKeys(): void
    {}

    private static function isTls13(
        #[\SensitiveParameter]
        Session $session
    ): bool {}

    /**
     * Returns the absolute expiry timestamp, or null when the session must
     * not be cached.
     */
    private static function expiry(
        #[\SensitiveParameter]
        Session $session
    ): ?int {}

    /**
     * Decides the absolute expiry timestamp from a session's scalar lifetime
     * attributes, or null when the session must not be cached.
     *
     * A non-positive session timeout is never cached. TLS 1.3 resumption
     * requires a New Session Ticket, so a TLS 1.3 session without a ticket
     * or without a positive RFC 8446 ticket lifetime is not cached, and the
     * ticket lifetime bounds the expiry together with the session timeout
     * and the seven-day cap. Pre-TLS-1.3 tickets treat a zero or missing
     * lifetime hint as unspecified per RFC 5077, so only a positive hint
     * tightens the session timeout and the one-day cap. An expiry that has
     * already passed is not cached.
     */
    private static function expiryFromLifetimes(bool $isTls13, bool $hasTicket, ?int $ticketLifetimeHint, int $timeout, int $createdAt, int $now): ?int
    {}

    private static function runtimeIdentity(): array
    {}

    /**
     * Returns the host identity hashed into the cache key. Valid bracketed
     * IPv6 literals are canonicalized to their RFC 5952 form so equivalent
     * spellings of one address share a single session entry; other valid
     * hosts, such as reg-names and IPvFuture literals, fall back to ASCII
     * case folding. Text that is not a valid RFC 3986 host, such as
     * zone-bearing or malformed bracketed literals, must never become a
     * usable cache key, so it is rejected.
     */
    private static function canonicalHost(string $host): string
    {}

    private static function canonicalPeerSslContext(
        #[\SensitiveParameter]
        array $ssl
    ): array {}

    private static function pathIdentity(string $path): array
    {}

    /**
     * @param mixed $value
     */
    private static function isCanonicalScalar($value): bool
    {}

    /**
     * @param mixed $value
     */
    private static function isPeerFingerprint($value): bool
    {}

    /**
     * @param mixed $value
     *
     * @return array{0: string, 1: mixed}
     */
    private static function canonicalScalar($value): array
    {}

    /**
     * @param mixed $value
     *
     * @return array{0: string, 1: mixed}
     */
    private static function canonicalPeerFingerprint($value): array
    {}
}
