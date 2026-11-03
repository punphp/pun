<?php

declare(strict_types=1);

namespace GuzzleHttp\Auth;

use GuzzleHttp\Psr7;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;

/**
 * @internal
 */
final class DigestAuth
{
    /**
     * @var array<string, array{hash: string, sess: bool, rank: int, header: string}>
     */
    private const ALGORITHMS = [
        'MD5' => ['hash' => 'md5', 'sess' => false, 'rank' => 10, 'header' => 'MD5'],
        'MD5-SESS' => ['hash' => 'md5', 'sess' => true, 'rank' => 11, 'header' => 'MD5-sess'],
        'SHA-256' => ['hash' => 'sha256', 'sess' => false, 'rank' => 20, 'header' => 'SHA-256'],
        'SHA-256-SESS' => ['hash' => 'sha256', 'sess' => true, 'rank' => 21, 'header' => 'SHA-256-sess'],
        'SHA-512-256' => ['hash' => 'sha512/256', 'sess' => false, 'rank' => 30, 'header' => 'SHA-512-256'],
        'SHA-512-256-SESS' => ['hash' => 'sha512/256', 'sess' => true, 'rank' => 31, 'header' => 'SHA-512-256-sess'],
    ];

    /**
     * @var array<string, true>
     */
    private const DIGEST_CHALLENGE_PARAMETER_NAMES = [
        'realm' => true,
        'domain' => true,
        'nonce' => true,
        'opaque' => true,
        'stale' => true,
        'algorithm' => true,
        'qop' => true,
        'charset' => true,
        'userhash' => true,
    ];

    private function __construct()
    {}

    public static function selectChallenge(
        #[\SensitiveParameter]
        ResponseInterface $response
    ): ?DigestChallenge {}

    public static function authorizationHeader(
        #[\SensitiveParameter]
        RequestInterface $request,
        DigestChallenge $challenge,
        string $username,
        #[\SensitiveParameter]
        string $password,
        string $cnonce,
        string $nc = '00000001'
    ): ?string {}

    /**
     * @return list<array{scheme: string, params: array<string, string>, invalid: bool}>
     */
    public static function parseAuthenticateHeader(string $header): array
    {}

    /**
     * @param array<string, string> $params
     */
    private static function createChallenge(array $params): ?DigestChallenge
    {}

    /**
     * @return array{name: string, hash: string, sess: bool, rank: int, header: string}|null
     */
    private static function algorithm(?string $algorithm): ?array
    {}

    /**
     * @return string|false|null
     */
    private static function selectQop(?string $qop)
    {}

    private static function quote(string $value): string
    {}

    private static function isHeaderSafe(string $value): bool
    {}

    private static function isNonceCount(string $nc): bool
    {}

    private static function commaStartsNextChallenge(string $header, int $offset, int $length): bool
    {}

    private static function commaStartsKnownDigestParameterWithoutValue(string $header, int $offset, int $length): bool
    {}

    private static function skipToken68Challenge(string $header, int &$offset, int $length): bool
    {}

    private static function skipSeparators(string $header, int &$offset, int $length): void
    {}

    private static function skipWhitespace(string $header, int &$offset, int $length): void
    {}

    private static function isWhitespace(string $char): bool
    {}

    private static function readToken(string $header, int &$offset, int $length): ?string
    {}

    private static function isTokenChar(string $char): bool
    {}

    private static function isToken68Char(string $char): bool
    {}

    private static function readValue(string $header, int &$offset, int $length): ?string
    {}

    private static function readQuotedString(string $header, int &$offset, int $length): ?string
    {}
}
