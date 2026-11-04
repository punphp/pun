<?php

namespace Illuminate\Http;

use Illuminate\Contracts\Support\MessageProvider;
use Illuminate\Contracts\Support\Responsable;
use Illuminate\Session\Store as SessionStore;
use Illuminate\Support\MessageBag;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\ForwardsCalls;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Uri;
use Illuminate\Support\ViewErrorBag;
use Symfony\Component\HttpFoundation\File\UploadedFile as SymfonyUploadedFile;
use Symfony\Component\HttpFoundation\RedirectResponse as BaseRedirectResponse;

class RedirectResponse extends BaseRedirectResponse implements Responsable
{
    use ForwardsCalls, ResponseTrait, Macroable {
        Macroable::__call as macroCall;
    }

    /**
     * The request instance.
     *
     * @var \Illuminate\Http\Request
     */
    protected $request;

    /**
     * The session store instance.
     *
     * @var \Illuminate\Session\Store
     */
    protected $session;

    /**
     * Flash a piece of data to the session.
     *
     * @param  string|array  $key
     * @param  mixed  $value
     * @return $this
     */
    public function with($key, $value = null)
    {}

    /**
     * Flash an array of input to the session.
     *
     * @param  array|null  $input
     * @return $this
     */
    public function withInput(?array $input = null)
    {}

    /**
     * Remove all uploaded files form the given input array.
     *
     * @param  array  $input
     * @return array
     */
    protected function removeFilesFromInput(array $input)
    {}

    /**
     * Flash an array of input to the session.
     *
     * @return $this
     */
    public function onlyInput()
    { func_get_args(); }

    /**
     * Flash an array of input to the session.
     *
     * @return $this
     */
    public function exceptInput()
    { func_get_args(); }

    /**
     * Flash a container of errors to the session.
     *
     * @param  \Illuminate\Contracts\Support\MessageProvider|array|string  $provider
     * @param  string  $key
     * @return $this
     */
    public function withErrors($provider, $key = 'default')
    {}

    /**
     * Parse the given errors into an appropriate value.
     *
     * @param  \Illuminate\Contracts\Support\MessageProvider|array|string  $provider
     * @return \Illuminate\Support\MessageBag
     */
    protected function parseErrors($provider)
    {}

    /**
     * Add a fragment identifier to the URL.
     *
     * @param  string  $fragment
     * @return $this
     */
    public function withFragment($fragment)
    {}

    /**
     * Remove any fragment identifier from the response URL.
     *
     * @return $this
     */
    public function withoutFragment()
    {}

    /**
     * Enforce that the redirect target must have the same host as the current request.
     */
    public function enforceSameOrigin(
        string $fallback,
        bool $validateScheme = true,
        bool $validatePort = true,
    ): static {}

    /**
     * Get the original response content.
     *
     * @return null
     */
    public function getOriginalContent()
    {}

    /**
     * Get the request instance.
     *
     * @return \Illuminate\Http\Request|null
     */
    public function getRequest()
    {}

    /**
     * Set the request instance.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return $this
     */
    public function setRequest(Request $request)
    {}

    /**
     * Get the session store instance.
     *
     * @return \Illuminate\Session\Store|null
     */
    public function getSession()
    {}

    /**
     * Set the session store instance.
     *
     * @param  \Illuminate\Session\Store  $session
     * @return $this
     */
    public function setSession(SessionStore $session)
    {}

    /**
     * Create an HTTP response that represents the object.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return $this
     */
    public function toResponse(mixed $request): static
    {}

    /**
     * Dynamically bind flash data in the session.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     *
     * @throws \BadMethodCallException
     */
    public function __call($method, $parameters)
    {}
}
