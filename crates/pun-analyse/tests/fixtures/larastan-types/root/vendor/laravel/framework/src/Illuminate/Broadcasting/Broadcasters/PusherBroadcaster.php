<?php

namespace Illuminate\Broadcasting\Broadcasters;

use Illuminate\Broadcasting\BroadcastException;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Pusher\ApiErrorException;
use Pusher\Pusher;
use Symfony\Component\HttpKernel\Exception\AccessDeniedHttpException;

class PusherBroadcaster extends Broadcaster
{
    use UsePusherChannelConventions;

    /**
     * The Pusher SDK instance.
     *
     * @var \Pusher\Pusher
     */
    protected $pusher;

    /**
     * Indicates if JSONP callbacks are allowed on authorization.
     *
     * @var bool
     */
    protected $allowJsonp = false;

    /**
     * Create a new broadcaster instance.
     *
     * @param  \Pusher\Pusher  $pusher
     */
    public function __construct(Pusher $pusher, bool $allowJsonp = false)
    {}

    /**
     * Resolve the authenticated user payload for an incoming connection request.
     *
     * See: https://pusher.com/docs/channels/library_auth_reference/auth-signatures/#user-authentication
     * See: https://pusher.com/docs/channels/server_api/authenticating-users/#response
     *
     * @param  \Illuminate\Http\Request  $request
     * @return array|null
     */
    public function resolveAuthenticatedUser($request)
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
     * Decode the given Pusher response.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  mixed  $response
     * @return array|\Illuminate\Http\JsonResponse
     */
    protected function decodePusherResponse($request, $response)
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
     * Get the Pusher SDK instance.
     *
     * @return \Pusher\Pusher
     */
    public function getPusher()
    {}

    /**
     * Set the Pusher SDK instance.
     *
     * @param  \Pusher\Pusher  $pusher
     * @return void
     */
    public function setPusher($pusher)
    {}
}
