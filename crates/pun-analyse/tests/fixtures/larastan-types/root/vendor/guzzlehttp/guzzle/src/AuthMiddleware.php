<?php

declare(strict_types=1);

namespace GuzzleHttp;

use GuzzleHttp\Auth\DigestAuth;
use GuzzleHttp\Auth\DigestChallenge;
use GuzzleHttp\Exception\InvalidArgumentException;
use GuzzleHttp\Exception\ResponseException;
use GuzzleHttp\Promise as P;
use GuzzleHttp\Promise\PromiseInterface;
use GuzzleHttp\Psr7\HttpFactory;
use GuzzleHttp\Psr7\LazyOpenStream;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Message\StreamFactoryInterface;
use Psr\Http\Message\StreamInterface;
use Psr\Http\Message\UriInterface;

/**
 * Applies built-in Basic authentication and handles Digest authentication
 * challenges.
 */
final class AuthMiddleware
{
    use NonSerializableTrait;

    private const DIGEST_MAX_RETRIES = 2;

    private const CHALLENGE_CACHE_LIMIT = 32;

    /**
     * Request option carrying nonce counts already spent during a Digest
     * handshake, so stale retries advance instead of repeating a count.
     */
    private const DIGEST_NONCE_COUNTS_OPTION = '__guzzle_digest_nonce_counts';

    /**
     * Headers that describe the request payload. A probe with an empty body
     * must not carry them.
     *
     * @var list<string>
     */
    private const DIGEST_PROBE_PAYLOAD_HEADERS = [
        'Transfer-Encoding',
        'Expect',
        'Trailer',
        'Content-Range',
        'Content-Encoding',
        'Content-MD5',
        'Digest',
        'Content-Digest',
        'Repr-Digest',
    ];

    /**
     * @var callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>
     */
    private $nextHandler;

    /**
     * @var callable(): string
     */
    private $cnonceGenerator;

    private bool $reuseChallenges;

    private ?string $credentialHashSecret = null;

    /**
     * @var array<string, array{challenge: DigestChallenge, credentials: string, nextNc: int}>
     */
    private array $digestChallenges = [];

    /**
     * @param callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed> $nextHandler
     * @param (callable(): string)|null                                                                       $cnonceGenerator
     */
    public function __construct(callable $nextHandler, ?callable $cnonceGenerator = null, bool $reuseChallenges = true)
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
     * @param array<array-key, mixed> $auth
     *
     * @return array{0: string, 1: string, 2: 'basic'|'digest'}|null
     */
    private static function normalizeAuth(
        #[\SensitiveParameter]
        array $auth
    ): ?array {}

    /**
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private function sendBasic(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options,
        string $username,
        #[\SensitiveParameter]
        string $password
    ): PromiseInterface {}

    /**
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private function sendDigest(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options,
        string $username,
        #[\SensitiveParameter]
        string $password
    ): PromiseInterface {}

    private function preemptiveDigestRequest(
        #[\SensitiveParameter]
        RequestInterface $request,
        string $username,
        #[\SensitiveParameter]
        string $password
    ): ?RequestInterface {}

    /**
     * Builds the unauthenticated probe request for a Digest handshake.
     *
     * @return array{0: RequestInterface, 1: bool}
     */
    private static function probeRequest(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ): array {}

    /**
     * @return ResponseInterface|PromiseInterface<ResponseInterface, mixed>
     */
    private function handleDigestResponse(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options,
        #[\SensitiveParameter]
        array $probeOptions,
        #[\SensitiveParameter]
        ResponseInterface $response,
        string $username,
        #[\SensitiveParameter]
        string $password,
        bool $bodyWithheld
    ) {}

    private static function isCacheableDigestSuccess(ResponseInterface $response): bool
    {}

    private function storeDigestChallenge(
        #[\SensitiveParameter]
        RequestInterface $request,
        DigestChallenge $challenge,
        string $username,
        #[\SensitiveParameter]
        string $password,
        int $nextNc
    ): void {}

    private function clearDigestChallenge(RequestInterface $request): void
    {}

    /**
     * @param array<array-key, mixed> $options
     *
     * @return array<string, int>
     */
    private static function digestNonceCountsFromOptions(array $options): array
    {}

    private static function digestNonceCountKey(DigestChallenge $challenge): string
    {}

    /**
     * @param array<string, int> $counts
     */
    private static function rememberDigestNonceCount(array &$counts, DigestChallenge $challenge, int $nextNc): void
    {}

    /**
     * @param mixed $reason
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private function handleDigestRejection(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options,
        #[\SensitiveParameter]
        $reason,
        bool $preemptive
    ): PromiseInterface {}

    private static function digestCacheKey(RequestInterface $request): ?string
    {}

    private function digestCredentialKey(
        string $username,
        #[\SensitiveParameter]
        string $password
    ): string {}

    private static function challengeCoversRequest(DigestChallenge $challenge, RequestInterface $request): bool
    {}

    private static function protectionSpacePath(string $space, RequestInterface $request): ?string
    {}

    private static function effectiveRequestUri(RequestInterface $request): UriInterface
    {}

    private static function isRedirectLikeResponse(ResponseInterface $response): bool
    {}

    private static function rewindBodyForRetry(
        #[\SensitiveParameter]
        RequestInterface $request,
        bool $bodyWithheld
    ): void {}

    private static function withTemporarySink(
        #[\SensitiveParameter]
        array $options
    ): array {}

    private static function restoreOriginalSink(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        ResponseInterface $response,
        #[\SensitiveParameter]
        array $options
    ): ResponseInterface {}

    /**
     * @param mixed $reason
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    private static function restoreOriginalSinkOnRejection(
        #[\SensitiveParameter]
        array $options,
        #[\SensitiveParameter]
        $reason
    ): PromiseInterface {}

    /**
     * @param mixed $sink
     */
    private static function streamForOriginalSink($sink): StreamInterface
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
}
