<?php

namespace Illuminate\Broadcasting\Broadcasters;

use Closure;
use Exception;
use Illuminate\Container\Container;
use Illuminate\Contracts\Broadcasting\Broadcaster as BroadcasterContract;
use Illuminate\Contracts\Broadcasting\HasBroadcastChannel;
use Illuminate\Contracts\Routing\BindingRegistrar;
use Illuminate\Contracts\Routing\UrlRoutable;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Reflector;
use ReflectionClass;
use ReflectionFunction;
use Symfony\Component\HttpKernel\Exception\AccessDeniedHttpException;

abstract class Broadcaster implements BroadcasterContract
{
    /**
     * The callback to resolve the authenticated user information.
     *
     * @var \Closure|null
     */
    protected $authenticatedUserCallback = null;

    /**
     * The registered channel authenticators.
     *
     * @var array
     */
    protected $channels = [];

    /**
     * The registered channel options.
     *
     * @var array
     */
    protected $channelOptions = [];

    /**
     * The binding registrar instance.
     *
     * @var \Illuminate\Contracts\Routing\BindingRegistrar|null
     */
    protected $bindingRegistrar;

    /**
     * Resolve the authenticated user payload for the incoming connection request.
     *
     * See: https://pusher.com/docs/channels/library_auth_reference/auth-signatures/#user-authentication.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return array|null
     */
    public function resolveAuthenticatedUser($request)
    {}

    /**
     * Register the user retrieval callback used to authenticate connections.
     *
     * See: https://pusher.com/docs/channels/library_auth_reference/auth-signatures/#user-authentication.
     *
     * @param  \Closure  $callback
     * @return void
     */
    public function resolveAuthenticatedUserUsing(Closure $callback)
    {}

    /**
     * Register a channel authenticator.
     *
     * @param  \Illuminate\Contracts\Broadcasting\HasBroadcastChannel|string  $channel
     * @param  callable|string  $callback
     * @param  array  $options
     * @return $this
     */
    public function channel($channel, $callback, $options = [])
    {}

    /**
     * Authenticate the incoming request for a given channel.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  string  $channel
     * @return mixed
     *
     * @throws \Symfony\Component\HttpKernel\Exception\AccessDeniedHttpException
     */
    protected function verifyUserCanAccessChannel($request, $channel)
    {}

    /**
     * Extract the parameters from the given pattern and channel.
     *
     * @param  string  $pattern
     * @param  string  $channel
     * @param  callable|string  $callback
     * @return array
     */
    protected function extractAuthParameters($pattern, $channel, $callback)
    {}

    /**
     * Extracts the parameters out of what the user passed to handle the channel authentication.
     *
     * @param  callable|string  $callback
     * @return \ReflectionParameter[]
     *
     * @throws \Exception
     */
    protected function extractParameters($callback)
    {}

    /**
     * Extracts the parameters out of a class channel's "join" method.
     *
     * @param  string  $callback
     * @return \ReflectionParameter[]
     *
     * @throws \Exception
     */
    protected function extractParametersFromClass($callback)
    {}

    /**
     * Extract the channel keys from the incoming channel name.
     *
     * @param  string  $pattern
     * @param  string  $channel
     * @return array
     */
    protected function extractChannelKeys($pattern, $channel)
    {}

    /**
     * Resolve the given parameter binding.
     *
     * @param  string  $key
     * @param  string  $value
     * @param  array  $callbackParameters
     * @return mixed
     */
    protected function resolveBinding($key, $value, $callbackParameters)
    {}

    /**
     * Resolve an explicit parameter binding if applicable.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return mixed
     */
    protected function resolveExplicitBindingIfPossible($key, $value)
    {}

    /**
     * Resolve an implicit parameter binding if applicable.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @param  array  $callbackParameters
     * @return mixed
     *
     * @throws \Symfony\Component\HttpKernel\Exception\AccessDeniedHttpException
     */
    protected function resolveImplicitBindingIfPossible($key, $value, $callbackParameters)
    {}

    /**
     * Determine if a given key and parameter is implicitly bindable.
     *
     * @param  string  $key
     * @param  \ReflectionParameter  $parameter
     * @return bool
     */
    protected function isImplicitlyBindable($key, $parameter)
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
     * Get the model binding registrar instance.
     *
     * @return \Illuminate\Contracts\Routing\BindingRegistrar|null
     */
    protected function binder()
    {}

    /**
     * Normalize the given callback into a callable.
     *
     * @param  mixed  $callback
     * @return callable
     */
    protected function normalizeChannelHandlerToCallable($callback)
    {}

    /**
     * Retrieve the authenticated user using the configured guard (if any).
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  string  $channel
     * @return mixed
     */
    protected function retrieveUser($request, $channel)
    {}

    /**
     * Retrieve options for a certain channel.
     *
     * @param  string  $channel
     * @return array
     */
    protected function retrieveChannelOptions($channel)
    {}

    /**
     * Check if the channel name from the request matches a pattern from registered channels.
     *
     * @param  string  $channel
     * @param  string  $pattern
     * @return bool
     */
    protected function channelNameMatchesPattern($channel, $pattern)
    {}

    /**
     * Get all of the registered channels.
     *
     * @return \Illuminate\Support\Collection
     */
    public function getChannels()
    {}
}
