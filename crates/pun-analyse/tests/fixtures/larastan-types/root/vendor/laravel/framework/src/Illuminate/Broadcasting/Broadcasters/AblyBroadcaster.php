<?php

namespace Illuminate\Broadcasting\Broadcasters;

use Ably\AblyRest;
use Ably\Exceptions\AblyException;
use Ably\Models\Message as AblyMessage;
use Illuminate\Broadcasting\BroadcastException;
use Illuminate\Support\Str;
use Symfony\Component\HttpKernel\Exception\AccessDeniedHttpException;

/**
 * @author Matthew Hall (matthall28@gmail.com)
 * @author Taylor Otwell (taylor@laravel.com)
 */
class AblyBroadcaster extends Broadcaster
{
    /**
     * The AblyRest SDK instance.
     *
     * @var \Ably\AblyRest
     */
    protected $ably;

    /**
     * Create a new broadcaster instance.
     *
     * @param  \Ably\AblyRest  $ably
     */
    public function __construct(AblyRest $ably)
    {}

    /**
     * Authenticate the incoming request for a given channel.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return mixed
     *
     * @throws \Symfony\Component\HttpKernel\Exception\AccessDeniedHttpException
     */
    public function auth($request)
    {}

    /**
     * Return the valid authentication response.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  mixed  $result
     * @return mixed
     */
    public function validAuthenticationResponse($request, $result)
    {}

    /**
     * Generate the signature needed for Ably authentication headers.
     *
     * @param  string  $channelName
     * @param  string  $socketId
     * @param  array|null  $userData
     * @return string
     */
    public function generateAblySignature($channelName, $socketId, $userData = null)
    {}

    /**
     * Broadcast the given event.
     *
     * @param  array  $channels
     * @param  string  $event
     * @param  array  $payload
     * @return void
     *
     * @throws \Illuminate\Broadcasting\BroadcastException
     */
    public function broadcast(array $channels, $event, array $payload = [])
    {}

    /**
     * Build an Ably message object for broadcasting.
     *
     * @param  string  $event
     * @param  array  $payload
     * @return \Ably\Models\Message
     */
    protected function buildAblyMessage($event, array $payload = [])
    {}

    /**
     * Return true if the channel is protected by authentication.
     *
     * @param  string  $channel
     * @return bool
     */
    public function isGuardedChannel($channel)
    {}

    /**
     * Remove prefix from channel name.
     *
     * @param  string  $channel
     * @return string
     */
    public function normalizeChannelName($channel)
    {}

    /**
     * Format the channel array into an array of strings.
     *
     * @param  array  $channels
     * @return array
     */
    protected function formatChannels(array $channels)
    {}

    /**
     * Get the public token value from the Ably key.
     *
     * @return string
     */
    protected function getPublicToken()
    {}

    /**
     * Get the private token value from the Ably key.
     *
     * @return string
     */
    protected function getPrivateToken()
    {}

    /**
     * Get the underlying Ably SDK instance.
     *
     * @return \Ably\AblyRest
     */
    public function getAbly()
    {}

    /**
     * Set the underlying Ably SDK instance.
     *
     * @param  \Ably\AblyRest  $ably
     * @return void
     */
    public function setAbly($ably)
    {}
}
