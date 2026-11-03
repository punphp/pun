<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\Exception\InvalidArgumentException;
use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\TransportSharing;

/**
 * @internal
 */
final class CurlShareHandleState
{
    use NonSerializableTrait;

    /**
     * @var resource|\CurlShareHandle|\CurlSharePersistentHandle|null
     */
    public $handle;

    public string $mode;

    /**
     * @param resource|\CurlShareHandle|\CurlSharePersistentHandle|null $handle
     */
    private function __construct(string $mode, $handle)
    {}

    /**
     * @param mixed $sharing
     */
    public static function fromOption($sharing): ?self
    {}

    /**
     * @param mixed $sharing
     */
    public static function normalizeMode($sharing, string $option): string
    {}

    public static function assertNoRequiredSharingCustomFactoryConflict(array $options, string $handlerName): void
    {}

    /**
     * Whether cURL can share both the DNS and SSL session cache state that
     * required handler-lifetime sharing needs, checked without configuring a
     * share handle so default handler selection can fall back to the stream
     * handler instead.
     */
    public static function supportsHandlerRequireShare(): bool
    {}

    private static function createHandlerShareOrNull(string $mode): ?self
    {}

    private static function createHandlerShare(string $mode): self
    {}

    private static function createPersistentShareOrFallback(): ?self
    {}

    private static function createPersistentShare(string $mode): self
    {}

    private static function supportsPersistentShare(): bool
    {}

    /**
     * @return int[]
     */
    private static function handlerLocks(string $mode): array
    {}

    /**
     * @return int[]
     */
    private static function persistentLocks(): array
    {}

    private static function requireCurlConstant(string $constant): int
    {}

    /**
     * @param resource|\CurlShareHandle $handle
     */
    private static function closeHandlerShareHandleOnPhp7($handle): void
    {}
}
