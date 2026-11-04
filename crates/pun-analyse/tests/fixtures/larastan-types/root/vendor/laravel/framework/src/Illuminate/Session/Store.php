<?php

namespace Illuminate\Session;

use BackedEnum;
use Closure;
use Illuminate\Contracts\Session\Session;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Date;
use Illuminate\Support\MessageBag;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Uri;
use Illuminate\Support\ViewErrorBag;
use RuntimeException;
use SessionHandlerInterface;
use stdClass;
use UnitEnum;

use function Illuminate\Support\enum_value;

class Store implements Session
{
    use Macroable;

    /**
     * The length of session ID strings.
     *
     * @var int
     */
    protected const SESSION_ID_LENGTH = 40;

    /**
     * The session ID.
     *
     * @var string
     */
    protected $id;

    /**
     * The session name.
     *
     * @var string
     */
    protected $name;

    /**
     * The session attributes.
     *
     * @var array
     */
    protected $attributes = [];

    /**
     * The session handler implementation.
     *
     * @var \SessionHandlerInterface
     */
    protected $handler;

    /**
     * The session store's serialization strategy.
     *
     * @var string
     */
    protected $serialization = 'php';

    /**
     * Session store started status.
     *
     * @var bool
     */
    protected $started = false;

    /**
     * Create a new session instance.
     *
     * @param  string  $name
     * @param  \SessionHandlerInterface  $handler
     * @param  string|null  $id
     * @param  string  $serialization
     */
    public function __construct($name, SessionHandlerInterface $handler, $id = null, $serialization = 'php')
    {}

    /**
     * Start the session, reading the data from a handler.
     *
     * @return bool
     */
    public function start()
    {}

    /**
     * Load the session data from the handler.
     *
     * @return void
     */
    protected function loadSession()
    {}

    /**
     * Read the session data from the handler.
     *
     * @return array
     */
    protected function readFromHandler()
    {}

    /**
     * Prepare the raw string data from the session for unserialization.
     *
     * @param  string  $data
     * @return string
     */
    protected function prepareForUnserialize($data)
    {}

    /**
     * Marshal the ViewErrorBag when using JSON serialization for sessions.
     *
     * @return void
     */
    protected function marshalErrorBag()
    {}

    /**
     * Save the session data to storage.
     *
     * @return void
     */
    public function save()
    {}

    /**
     * Prepare the ViewErrorBag instance for JSON serialization.
     *
     * @return void
     */
    protected function prepareErrorBagForSerialization()
    {}

    /**
     * Prepare the serialized session data for storage.
     *
     * @param  string  $data
     * @return string
     */
    protected function prepareForStorage($data)
    {}

    /**
     * Age the flash data for the session.
     *
     * @return void
     */
    public function ageFlashData()
    {}

    /**
     * Get all of the session data.
     *
     * @return array
     */
    public function all()
    {}

    /**
     * Get a subset of the session data.
     *
     * @param  array  $keys
     * @return array
     */
    public function only(array $keys)
    {}

    /**
     * Get all the session data except for a specified array of items.
     *
     * @param  array  $keys
     * @return array
     */
    public function except(array $keys)
    {}

    /**
     * Checks if a key exists.
     *
     * @param  \UnitEnum|string|array  $key
     * @return bool
     */
    public function exists($key)
    { func_get_args(); }

    /**
     * Determine if the given key is missing from the session data.
     *
     * @param  \UnitEnum|string|array  $key
     * @return bool
     */
    public function missing($key)
    {}

    /**
     * Determine if a key is present and not null.
     *
     * @param  \UnitEnum|string|array  $key
     * @return bool
     */
    public function has($key)
    { func_get_args(); }

    /**
     * Determine if any of the given keys are present and not null.
     *
     * @param  \UnitEnum|string|array  $key
     * @return bool
     */
    public function hasAny($key)
    { func_get_args(); }

    /**
     * Get an item from the session.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function get($key, $default = null)
    {}

    /**
     * Get the value of a given key and then forget it.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function pull($key, $default = null)
    {}

    /**
     * Determine if the session contains old input.
     *
     * @param  string|null  $key
     * @return bool
     */
    public function hasOldInput($key = null)
    {}

    /**
     * Get the requested item from the flashed input array.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function getOldInput($key = null, $default = null)
    {}

    /**
     * Replace the given session attributes entirely.
     *
     * @param  array  $attributes
     * @return void
     */
    public function replace(array $attributes)
    {}

    /**
     * Put a key / value pair or array of key / value pairs in the session.
     *
     * @param  \UnitEnum|string|array  $key
     * @param  mixed  $value
     * @return void
     */
    public function put($key, $value = null)
    {}

    /**
     * Get an item from the session, or store the default value.
     *
     * @param  \UnitEnum|string  $key
     * @param  \Closure  $callback
     * @return mixed
     */
    public function remember($key, Closure $callback)
    {}

    /**
     * Push a value onto a session array.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $value
     * @return void
     */
    public function push($key, $value)
    {}

    /**
     * Increment the value of an item in the session.
     *
     * @param  \UnitEnum|string  $key
     * @param  int  $amount
     * @return mixed
     */
    public function increment($key, $amount = 1)
    {}

    /**
     * Decrement the value of an item in the session.
     *
     * @param  \UnitEnum|string  $key
     * @param  int  $amount
     * @return int
     */
    public function decrement($key, $amount = 1)
    {}

    /**
     * Flash a key / value pair to the session.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $value
     * @return void
     */
    public function flash(BackedEnum|UnitEnum|string $key, $value = true)
    {}

    /**
     * Flash a key / value pair to the session for immediate use.
     *
     * @param  \UnitEnum|string  $key
     * @param  mixed  $value
     * @return void
     */
    public function now($key, $value)
    {}

    /**
     * Reflash all of the session flash data.
     *
     * @return void
     */
    public function reflash()
    {}

    /**
     * Reflash a subset of the current flash data.
     *
     * @param  mixed  $keys
     * @return void
     */
    public function keep($keys = null)
    { func_get_args(); }

    /**
     * Merge new flash keys into the new flash array.
     *
     * @param  array  $keys
     * @return void
     */
    protected function mergeNewFlashes(array $keys)
    {}

    /**
     * Remove the given keys from the old flash data.
     *
     * @param  array  $keys
     * @return void
     */
    protected function removeFromOldFlashData(array $keys)
    {}

    /**
     * Flash an input array to the session.
     *
     * @param  array  $value
     * @return void
     */
    public function flashInput(array $value)
    {}

    /**
     * Get the session cache instance.
     *
     * @return \Illuminate\Contracts\Cache\Repository
     */
    public function cache()
    {}

    /**
     * Remove an item from the session, returning its value.
     *
     * @param  \UnitEnum|string  $key
     * @return mixed
     */
    public function remove($key)
    {}

    /**
     * Remove one or many items from the session.
     *
     * @param  \UnitEnum|string|array  $keys
     * @return void
     */
    public function forget($keys)
    {}

    /**
     * Remove all of the items from the session.
     *
     * @return void
     */
    public function flush()
    {}

    /**
     * Flush the session data and regenerate the ID.
     *
     * @return bool
     */
    public function invalidate()
    {}

    /**
     * Generate a new session identifier.
     *
     * @param  bool  $destroy
     * @return bool
     */
    public function regenerate($destroy = false)
    {}

    /**
     * Generate a new session ID for the session.
     *
     * @param  bool  $destroy
     * @return bool
     */
    public function migrate($destroy = false)
    {}

    /**
     * Determine if the session has been started.
     *
     * @return bool
     */
    public function isStarted()
    {}

    /**
     * Get the name of the session.
     *
     * @return string
     */
    public function getName()
    {}

    /**
     * Set the name of the session.
     *
     * @param  string  $name
     * @return void
     */
    public function setName($name)
    {}

    /**
     * Get the current session ID.
     *
     * @return string
     */
    public function id()
    {}

    /**
     * Get the current session ID.
     *
     * @return string
     */
    public function getId()
    {}

    /**
     * Set the session ID.
     *
     * @param  string|null  $id
     * @return void
     */
    public function setId($id)
    {}

    /**
     * Determine if this is a valid session ID.
     *
     * @param  string|null  $id
     * @return bool
     */
    public function isValidId($id)
    {}

    /**
     * Get a new, random session ID.
     *
     * @return string
     */
    protected function generateSessionId()
    {}

    /**
     * Set the existence of the session on the handler if applicable.
     *
     * @param  bool  $value
     * @return void
     */
    public function setExists($value)
    {}

    /**
     * Get the CSRF token value.
     *
     * @return string
     */
    public function token()
    {}

    /**
     * Regenerate the CSRF token value.
     *
     * @return void
     */
    public function regenerateToken()
    {}

    /**
     * Determine if the previous URI is available.
     *
     * @return bool
     */
    public function hasPreviousUri()
    {}

    /**
     * Get the previous URL from the session as a URI instance.
     *
     * @return \Illuminate\Support\Uri
     *
     * @throws \RuntimeException
     */
    public function previousUri()
    {}

    /**
     * Get the previous URL from the session.
     *
     * @return string|null
     */
    public function previousUrl()
    {}

    /**
     * Set the "previous" URL in the session.
     *
     * @param  string  $url
     * @return void
     */
    public function setPreviousUrl($url)
    {}

    /**
     * Get the previous route name from the session.
     *
     * @return string|null
     */
    public function previousRoute()
    {}

    /**
     * Set the "previous" route name in the session.
     *
     * @param  string|null  $route
     * @return void
     */
    public function setPreviousRoute($route)
    {}

    /**
     * Specify that the user has confirmed their password.
     *
     * @return void
     */
    public function passwordConfirmed()
    {}

    /**
     * Get the underlying session handler implementation.
     *
     * @return \SessionHandlerInterface
     */
    public function getHandler()
    {}

    /**
     * Set the underlying session handler implementation.
     *
     * @param  \SessionHandlerInterface  $handler
     * @return \SessionHandlerInterface
     */
    public function setHandler(SessionHandlerInterface $handler)
    {}

    /**
     * Determine if the session handler needs a request.
     *
     * @return bool
     */
    public function handlerNeedsRequest()
    {}

    /**
     * Set the request on the handler instance.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return void
     */
    public function setRequestOnHandler($request)
    {}
}
