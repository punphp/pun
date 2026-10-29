<?php

namespace Probe;

final class Mercure
{
    /**
     * @param  string[]|null  $channels
     * @param  string  $event
     * @param  array<string, mixed>  $payload
     * @param  string|null  $socket
     */
    protected function updateData($channels, $event, array $payload, $socket)
    {
        return array_filter([
            'channels' => $channels,
            'event' => $event,
            'payload' => $payload,
            'socket' => $socket,
        ], fn ($value) => $value !== null);
    }

    public function mixedOrder(array $a, int $b, ?string $c)
    {
        return array_filter([$a, $b, 1, $c], fn ($value) => $value !== null);
    }
}
-----
<?php

namespace Probe;

final class Mercure
{
    /**
     * @param  string[]|null  $channels
     * @param  string  $event
     * @param  array<string, mixed>  $payload
     * @param  string|null  $socket
     */
    protected function updateData($channels, $event, array $payload, $socket)
    {
        return array_filter([
            'channels' => $channels,
            'event' => $event,
            'payload' => $payload,
            'socket' => $socket,
        ], fn (string|array|null $value) => $value !== null);
    }

    public function mixedOrder(array $a, int $b, ?string $c)
    {
        return array_filter([$a, $b, 1, $c], fn (int|string|array|null $value) => $value !== null);
    }
}
