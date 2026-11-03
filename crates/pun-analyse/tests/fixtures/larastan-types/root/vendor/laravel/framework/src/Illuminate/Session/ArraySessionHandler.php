<?php

namespace Illuminate\Session;

use Illuminate\Support\InteractsWithTime;
use RuntimeException;
use SessionHandlerInterface;

class ArraySessionHandler implements SessionHandlerInterface
{
    use InteractsWithTime;

    /**
     * The array of stored values.
     *
     * @var array
     */
    protected $storage = [];

    /**
     * The number of minutes the session should be valid.
     *
     * @var int
     */
    protected $minutes;

    /**
     * Create a new array driven handler instance.
     *
     * @param  int  $minutes
     */
    public function __construct($minutes)
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
     * @return string|false
     */
    public function read($sessionId): string|false
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
     * Get the expiration time of the session.
     *
     * @param  int  $seconds
     * @return int
     */
    protected function calculateExpiration($seconds)
    {}
}
