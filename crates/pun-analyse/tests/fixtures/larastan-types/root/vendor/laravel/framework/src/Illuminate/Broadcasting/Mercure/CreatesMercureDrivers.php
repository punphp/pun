<?php

namespace Illuminate\Broadcasting\Mercure;

use Illuminate\Broadcasting\Broadcasters\MercureBroadcaster;
use InvalidArgumentException;
use Symfony\Component\HttpClient\HttpClient;
use Symfony\Component\Mercure\FrankenPhpHub;
use Symfony\Component\Mercure\Hub;
use Symfony\Component\Mercure\Jwt\DefaultClaimsTokenFactory;
use Symfony\Component\Mercure\Jwt\FactoryTokenProvider;
use Symfony\Component\Mercure\Jwt\Grant;
use Symfony\Component\Mercure\Jwt\WebTokenFactory;
use Symfony\Component\Mercure\ProtocolVersion;

use function Illuminate\Support\enum_value;

trait CreatesMercureDrivers
{
    /**
     * Create an instance of the driver.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Broadcasting\Broadcaster
     */
    protected function createMercureDriver(array $config)
    {}

    /**
     * Get a Mercure hub instance for the given configuration.
     *
     * @param  array  $config
     * @return \Symfony\Component\Mercure\HubInterface
     */
    public function mercure(array $config)
    {}

    /**
     * Get a Mercure hub instance backed by FrankenPHP's built-in hub.
     *
     * @param  array  $config
     * @return \Symfony\Component\Mercure\HubInterface
     */
    protected function frankenPhpMercure(array $config)
    {}

    /**
     * Determine if the given event only targets Mercure connections publishing synchronously.
     *
     * @param  mixed  $event
     * @return bool
     */
    protected function broadcastsToSynchronousMercureConnections($event)
    {}

    /**
     * Get the subscriber token (and cookie) lifetime in whole seconds.
     *
     * @param  array  $config
     * @return int
     */
    protected function mercureSubscribeExpiration(array $config)
    {}

    /**
     * Get the subscriber token factory for the given Mercure configuration.
     *
     * @param  array  $config
     * @return \Symfony\Component\Mercure\Jwt\TokenFactoryInterface|null
     */
    protected function mercureSubscribeFactory(array $config)
    {}

    /**
     * Get the end-to-end channel encrypter for the given Mercure configuration.
     *
     * @param  array  $config
     * @return \Illuminate\Broadcasting\Mercure\ChannelEncrypter|null
     */
    protected function mercureChannelEncrypter(array $config)
    {}

    /**
     * Get the additional JWT claims for the publisher side of the given Mercure configuration.
     *
     * @param  array  $config
     * @return array
     */
    protected function mercurePublishClaims(array $config)
    {}

    /**
     * Get the JWT secret for the given side ("subscribe" or "publish") of the given Mercure configuration.
     *
     * @param  array  $config
     * @param  string  $side
     * @return string
     */
    protected function mercureSecret(array $config, string $side)
    {}

    /**
     * Get the additional JWT claims for the given Mercure configuration.
     *
     * @param  array  $config
     * @return array
     */
    protected function mercureClaims(array $config)
    {}
}
