<?php

namespace Illuminate\Session;

use RuntimeException;
use SessionHandlerInterface;

class NullSessionHandler implements SessionHandlerInterface
{
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
}
