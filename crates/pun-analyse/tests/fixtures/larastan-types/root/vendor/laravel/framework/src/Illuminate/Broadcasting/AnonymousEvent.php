<?php

namespace Illuminate\Broadcasting;

use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Foundation\Events\Dispatchable;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;

class AnonymousEvent implements ShouldBroadcast
{
    use Dispatchable, InteractsWithBroadcasting, InteractsWithSockets;

    /**
     * The connection the event should be broadcast on.
     */
    protected ?string $connection = null;

    /**
     * The name the event should be broadcast as.
     */
    protected ?string $name = null;

    /**
     * The payload the event should be broadcast with.
     */
    protected array $payload = [];

    /**
     * Should the broadcast include the current user.
     */
    protected bool $includeCurrentUser = true;

    /**
     * Indicates if the event should be broadcast synchronously.
     */
    protected bool $shouldBroadcastNow = false;

    /**
     * Create a new anonymous broadcastable event instance.
     */
    public function __construct(protected Channel|array|string $channels)
    {}

    /**
     * Set the connection the event should be broadcast on.
     */
    public function via(string $connection): static
    {}

    /**
     * Set the name the event should be broadcast as.
     */
    public function as(string $name): static
    {}

    /**
     * Set the payload the event should be broadcast with.
     */
    public function with(Arrayable|array $payload): static
    {}

    /**
     * Broadcast the event to everyone except the current user.
     */
    public function toOthers(): static
    {}

    /**
     * Broadcast the event.
     */
    public function sendNow(): void
    {}

    /**
     * Broadcast the event.
     */
    public function send(): void
    {}

    /**
     * Get the name the event should broadcast as.
     */
    public function broadcastAs(): string
    {}

    /**
     * Get the payload the event should broadcast with.
     *
     * @return array<string, mixed>
     */
    public function broadcastWith(): array
    {}

    /**
     * Get the channels the event should broadcast on.
     *
     * @return \Illuminate\Broadcasting\Channel|\Illuminate\Broadcasting\Channel[]|string[]
     */
    public function broadcastOn(): Channel|array
    {}

    /**
     * Determine if the event should be broadcast synchronously.
     */
    public function shouldBroadcastNow(): bool
    {}
}
