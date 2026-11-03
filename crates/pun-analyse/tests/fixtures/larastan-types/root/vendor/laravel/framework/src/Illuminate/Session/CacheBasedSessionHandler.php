<?php

namespace Illuminate\Session;

use Illuminate\Contracts\Cache\Repository as CacheContract;
use RuntimeException;
use SessionHandlerInterface;

class CacheBasedSessionHandler implements SessionHandlerInterface
{
    /**
     * The cache repository instance.
     *
     * @var \Illuminate\Contracts\Cache\Repository
     */
    protected $cache;

    /**
     * The number of minutes to store the data in the cache.
     *
     * @var int
     */
    protected $minutes;

    /**
     * Create a new cache driven handler instance.
     *
     * @param  \Illuminate\Contracts\Cache\Repository  $cache
     * @param  int  $minutes
     */
    public function __construct(CacheContract $cache, $minutes)
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    public function open($savePath, $sessionName): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    public function close(): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @return string
     */
    public function create_sid(): string
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    public function validateId($id): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @return string
     */
    public function read($sessionId): string
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    public function write($sessionId, $data): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    public function destroy($sessionId): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @return int
     */
    public function gc($lifetime): int
    {}

    /**
     * Get the underlying cache repository.
     *
     * @return \Illuminate\Contracts\Cache\Repository
     */
    public function getCache()
    {}
}
