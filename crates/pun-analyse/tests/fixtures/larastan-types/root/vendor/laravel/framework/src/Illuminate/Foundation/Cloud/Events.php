<?php

namespace Illuminate\Foundation\Cloud;

use RuntimeException;
use Throwable;

class Events
{
    /**
     * The cloud socket.
     *
     * @var resource|null
     */
    protected $socket = null;

    /**
     * The cloud socket factory.
     *
     * @var callable|null
     */
    public static $socketFactory = null;

    /**
     * Create a new instance.
     */
    public function __construct(protected string $address)
    {}

    /**
     * Emit an event.
     *
     * @param  array<string, mixed>  $payload
     */
    public function emit(array $payload): bool
    {}

    /**
     * Emit many events.
     *
     * @param  list<array<string, mixed>>  $payloads
     */
    public function emitMany(array $payloads): bool
    {}

    /**
     * Write the payload to the socket.
     */
    protected function write(string $payload): void
    {}

    /**
     * Format the payload.
     *
     * @param  list<array<string, mixed>>  $payloads
     *
     * @throws \JsonException
     */
    protected function format(array $payloads): string
    {}

    /**
     * Ensure the socket is connected.
     */
    protected function ensureConnected(): void
    {}

    /**
     * Connect the socket.
     */
    protected function connect(): void
    {}

    /**
     * Determine if the socket is connected.
     */
    protected function connected(): bool
    {}

    /**
     * Disconnect the socket.
     */
    protected function disconnect(): void
    {}

    /**
     * Decorate the message with the socket's meta data.
     */
    protected function withSocketMetaData(string $message): string
    {}
}
