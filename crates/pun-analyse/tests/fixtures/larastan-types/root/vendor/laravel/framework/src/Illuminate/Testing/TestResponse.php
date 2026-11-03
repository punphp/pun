<?php

namespace Illuminate\Testing;

use ArrayAccess;
use Closure;
use Illuminate\Contracts\Support\MessageBag;
use Illuminate\Contracts\View\View;
use Illuminate\Cookie\CookieValuePrefix;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Http\Request;
use Illuminate\Support\Arr;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Dumpable;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\Tappable;
use Illuminate\Support\ViewErrorBag;
use Illuminate\Testing\Constraints\SeeInHtml;
use Illuminate\Testing\Constraints\SeeInOrder;
use Illuminate\Testing\Fluent\AssertableJson;
use Illuminate\Testing\TestResponseAssert as PHPUnit;
use LogicException;
use Symfony\Component\HttpFoundation\BinaryFileResponse;
use Symfony\Component\HttpFoundation\Cookie;
use Symfony\Component\HttpFoundation\StreamedJsonResponse;
use Symfony\Component\HttpFoundation\StreamedResponse;

/**
 * @template TResponse of \Symfony\Component\HttpFoundation\Response
 *
 * @mixin \Illuminate\Http\Response
 */
class TestResponse implements ArrayAccess
{
    use Concerns\AssertsStatusCodes, Conditionable, Dumpable, Tappable, Macroable {
        __call as macroCall;
    }

    /**
     * The original request.
     *
     * @var \Illuminate\Http\Request|null
     */
    public $baseRequest;

    /**
     * The response to delegate to.
     *
     * @var TResponse
     */
    public $baseResponse;

    /**
     * The collection of logged exceptions for the request.
     *
     * @var \Illuminate\Support\Collection
     */
    public $exceptions;

    /**
     * The streamed content of the response.
     *
     * @var string|null
     */
    protected $streamedContent;

    /**
     * The decoded response JSON.
     *
     * @var \Illuminate\Testing\AssertableJsonString|null
     */
    protected $decodedResponseJson;

    /**
     * Create a new test response instance.
     *
     * @param  TResponse  $response
     * @param  \Illuminate\Http\Request|null  $request
     */
    public function __construct($response, $request = null)
    {}

    /**
     * Create a new TestResponse from another response.
     *
     * @template R of TResponse
     *
     * @param  R  $response
     * @param  \Illuminate\Http\Request|null  $request
     * @return static<R>
     */
    public static function fromBaseResponse($response, $request = null)
    {}

    /**
     * Assert that the response has a successful status code.
     *
     * @return $this
     */
    public function assertSuccessful()
    {}

    /**
     * Assert that the Precognition request was successful.
     *
     * @return $this
     */
    public function assertSuccessfulPrecognition()
    {}

    /**
     * Assert that the response is a client error.
     *
     * @return $this
     */
    public function assertClientError()
    {}

    /**
     * Assert that the response is a server error.
     *
     * @return $this
     */
    public function assertServerError()
    {}

    /**
     * Assert that the response has the given status code.
     *
     * @param  int  $status
     * @return $this
     */
    public function assertStatus($status)
    {}

    /**
     * Get an assertion message for a status assertion containing extra details when available.
     *
     * @param  string|int  $expected
     * @param  string|int  $actual
     * @return string
     */
    protected function statusMessageWithDetails($expected, $actual)
    {}

    /**
     * Assert whether the response is redirecting to a given URI.
     *
     * @param  string|null  $uri
     * @return $this
     */
    public function assertRedirect($uri = null)
    {}

    /**
     * Assert whether the response is redirecting to a URI that contains the given URI.
     *
     * @param  string  $uri
     * @return $this
     */
    public function assertRedirectContains($uri)
    {}

    /**
     * Assert whether the response is redirecting back to the previous location.
     *
     * @return $this
     */
    public function assertRedirectBack()
    {}

    /**
     * Assert whether the response is redirecting back to the previous location with the given errors in the session.
     *
     * @param  string|array  $keys
     * @param  mixed  $format
     * @param  string  $errorBag
     * @return $this
     */
    public function assertRedirectBackWithErrors($keys = [], $format = null, $errorBag = 'default')
    {}

    /**
     * Assert whether the response is redirecting back to the previous location with no errors in the session.
     *
     * @return $this
     */
    public function assertRedirectBackWithoutErrors()
    {}

    /**
     * Assert whether the response is redirecting to a given route.
     *
     * @param  \BackedEnum|string  $name
     * @param  mixed  $parameters
     * @return $this
     */
    public function assertRedirectToRoute($name, $parameters = [])
    {}

    /**
     * Assert whether the response is redirecting to a given signed route.
     *
     * @param  \BackedEnum|string|null  $name
     * @param  mixed  $parameters
     * @param  bool  $absolute
     * @return $this
     */
    public function assertRedirectToSignedRoute($name = null, $parameters = [], $absolute = true)
    {}

    /**
     * Assert whether the response is redirecting to a given controller action.
     *
     * @param  string|array  $name
     * @param  array  $parameters
     * @return $this
     */
    public function assertRedirectToAction($name, $parameters = [])
    {}

    /**
     * Asserts that the response contains the given header and equals the optional value.
     *
     * @param  string  $headerName
     * @param  mixed  $value
     * @return $this
     */
    public function assertHeader($headerName, $value = null)
    {}

    /**
     * Asserts that the response contains the given header and that its value contains the given string.
     *
     * @param  string  $headerName
     * @param  string  $value
     * @return $this
     */
    public function assertHeaderContains($headerName, $value)
    {}

    /**
     * Asserts that the response does not contain the given header.
     *
     * @param  string  $headerName
     * @return $this
     */
    public function assertHeaderMissing($headerName)
    {}

    /**
     * Assert that the current location header matches the given URI.
     *
     * @param  string  $uri
     * @return $this
     */
    public function assertLocation($uri)
    {}

    /**
     * Assert that the response offers a file download.
     *
     * @param  string|null  $filename
     * @return $this
     */
    public function assertDownload($filename = null)
    {}

    /**
     * Asserts that the response contains the given cookie and equals the optional value.
     *
     * @param  string  $cookieName
     * @param  mixed  $value
     * @return $this
     */
    public function assertPlainCookie($cookieName, $value = null)
    {}

    /**
     * Asserts that the response contains the given cookie and equals the optional value.
     *
     * @param  string  $cookieName
     * @param  mixed  $value
     * @param  bool  $encrypted
     * @param  bool  $unserialize
     * @return $this
     */
    public function assertCookie($cookieName, $value = null, $encrypted = true, $unserialize = false)
    {}

    /**
     * Asserts that the response contains the given cookie and is expired.
     *
     * @param  string  $cookieName
     * @return $this
     */
    public function assertCookieExpired($cookieName)
    {}

    /**
     * Asserts that the response contains the given cookie and is not expired.
     *
     * @param  string  $cookieName
     * @return $this
     */
    public function assertCookieNotExpired($cookieName)
    {}

    /**
     * Asserts that the response does not contain the given cookie.
     *
     * @param  string  $cookieName
     * @return $this
     */
    public function assertCookieMissing($cookieName)
    {}

    /**
     * Get the given cookie from the response.
     *
     * @param  string  $cookieName
     * @param  bool  $decrypt
     * @param  bool  $unserialize
     * @return \Symfony\Component\HttpFoundation\Cookie|null
     */
    public function getCookie($cookieName, $decrypt = true, $unserialize = false)
    {}

    /**
     * Assert that the given string matches the response content.
     *
     * @param  string  $value
     * @return $this
     */
    public function assertContent($value)
    {}

    /**
     * Assert that the response was streamed.
     *
     * @return $this
     */
    public function assertStreamed()
    {}

    /**
     * Assert that the response was not streamed.
     *
     * @return $this
     */
    public function assertNotStreamed()
    {}

    /**
     * Assert that the given string matches the streamed response content.
     *
     * @param  string  $value
     * @return $this
     */
    public function assertStreamedContent($value)
    {}

    /**
     * Assert that the given array matches the streamed JSON response content.
     *
     * @param  array  $value
     * @return $this
     *
     * @throws \JsonException
     */
    public function assertStreamedJsonContent($value)
    {}

    /**
     * Assert that the given string or array of strings are contained within the response.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertSee($value, $escape = true)
    {}

    /**
     * Assert that the given HTML string or array of HTML strings are contained within the response.
     *
     * @param  string|list<string>  $value
     * @return $this
     */
    public function assertSeeHtml($value)
    {}

    /**
     * Assert that the given strings are contained in order within the response.
     *
     * @param  list<string>  $values
     * @param  bool  $escape
     * @return $this
     */
    public function assertSeeInOrder(array $values, $escape = true)
    {}

    /**
     * Assert that the given HTML strings are contained in order within the response.
     *
     * @param  list<string>  $values
     * @return $this
     */
    public function assertSeeHtmlInOrder(array $values)
    {}

    /**
     * Assert that the given string or array of strings are contained within the response text.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertSeeText($value, $escape = true)
    {}

    /**
     * Assert that the given strings are contained in order within the response text.
     *
     * @param  list<string>  $values
     * @param  bool  $escape
     * @return $this
     */
    public function assertSeeTextInOrder(array $values, $escape = true)
    {}

    /**
     * Assert that the given string or array of strings are not contained within the response.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertDontSee($value, $escape = true)
    {}

    /**
     * Assert that the given HTML string or array of HTML strings are not contained within the response.
     *
     * @param  string|list<string>  $value
     * @return $this
     */
    public function assertDontSeeHtml($value)
    {}

    /**
     * Assert that the given string or array of strings are not contained within the response text.
     *
     * @param  string|list<string>  $value
     * @param  bool  $escape
     * @return $this
     */
    public function assertDontSeeText($value, $escape = true)
    {}

    /**
     * Assert that the response is a superset of the given JSON.
     *
     * @param  array|callable  $value
     * @param  bool  $strict
     * @return $this
     */
    public function assertJson($value, $strict = false)
    {}

    /**
     * Assert that the expected value and type exists at the given path in the response.
     *
     * @param  string  $path
     * @param  mixed  $expect
     * @return $this
     */
    public function assertJsonPath($path, $expect)
    {}

    /**
     * Assert that the expected values and types exist at the given paths in the response.
     *
     * @return $this
     */
    public function assertJsonPaths(array $paths)
    {}

    /**
     * Assert that the given paths in the response contain all of the expected values without looking at the order.
     *
     * @return $this
     */
    public function assertJsonPathsCanonicalizing(array $paths)
    {}

    /**
     * Assert that the given path in the response contains all of the expected values without looking at the order.
     *
     * @param  string  $path
     * @param  array  $expect
     * @return $this
     */
    public function assertJsonPathCanonicalizing($path, array $expect)
    {}

    /**
     * Assert that the response has the exact given JSON.
     *
     * @param  array  $data
     * @return $this
     */
    public function assertExactJson(array $data)
    {}

    /**
     * Assert that the response has the similar JSON as given.
     *
     * @param  array  $data
     * @return $this
     */
    public function assertSimilarJson(array $data)
    {}

    /**
     * Assert that the response contains the given JSON fragments.
     *
     * @param  array  $data
     * @return $this
     */
    public function assertJsonFragments(array $data)
    {}

    /**
     * Assert that the response contains the given JSON fragment.
     *
     * @param  array  $data
     * @return $this
     */
    public function assertJsonFragment(array $data)
    {}

    /**
     * Assert that the response does not contain the given JSON fragment.
     *
     * @param  array  $data
     * @param  bool  $exact
     * @return $this
     */
    public function assertJsonMissing(array $data, $exact = false)
    {}

    /**
     * Assert that the response does not contain the exact JSON fragment.
     *
     * @param  array  $data
     * @return $this
     */
    public function assertJsonMissingExact(array $data)
    {}

    /**
     * Assert that the response does not contain the given path.
     *
     * @param  string  $path
     * @return $this
     */
    public function assertJsonMissingPath(string $path)
    {}

    /**
     * Assert that the response does not contain the given paths.
     *
     * @return $this
     */
    public function assertJsonMissingPaths(array $paths)
    {}

    /**
     * Assert that the response has a given JSON structure.
     *
     * @param  array|null  $structure
     * @param  array|null  $responseData
     * @return $this
     */
    public function assertJsonStructure(?array $structure = null, ?array $responseData = null)
    {}

    /**
     * Assert that the response has the exact JSON structure.
     *
     * @param  array|null  $structure
     * @param  array|null  $responseData
     * @return $this
     */
    public function assertExactJsonStructure(?array $structure = null, ?array $responseData = null)
    {}

    /**
     * Assert that the response JSON has the expected count of items at the given key.
     *
     * @param  int  $count
     * @param  string|null  $key
     * @return $this
     */
    public function assertJsonCount(int $count, $key = null)
    {}

    /**
     * Assert that the response has the given JSON validation errors.
     *
     * @param  string|array  $errors
     * @param  string  $responseKey
     * @return $this
     */
    public function assertJsonValidationErrors($errors, $responseKey = 'errors')
    {}

    /**
     * Assert that the response has the given JSON validation errors but does not have any other JSON validation errors.
     *
     * @param  string|array  $errors
     * @param  string  $responseKey
     * @return $this
     */
    public function assertOnlyJsonValidationErrors($errors, $responseKey = 'errors')
    {}

    /**
     * Assert the response has any JSON validation errors for the given key.
     *
     * @param  string  $key
     * @param  string  $responseKey
     * @return $this
     */
    public function assertJsonValidationErrorFor($key, $responseKey = 'errors')
    {}

    /**
     * Assert that the response has no JSON validation errors for the given keys.
     *
     * @param  string|array|null  $keys
     * @param  string  $responseKey
     * @return $this
     */
    public function assertJsonMissingValidationErrors($keys = null, $responseKey = 'errors')
    {}

    /**
     * Assert that the given key is a JSON array.
     *
     * @param  string|null  $key
     * @return $this
     */
    public function assertJsonIsArray($key = null)
    {}

    /**
     * Assert that the given key is a JSON object.
     *
     * @param  string|null  $key
     * @return $this
     */
    public function assertJsonIsObject($key = null)
    {}

    /**
     * Validate the decoded response JSON.
     *
     * @return \Illuminate\Testing\AssertableJsonString
     *
     * @throws \Throwable
     */
    public function decodeResponseJson()
    {}

    /**
     * Return the decoded response JSON.
     *
     * @param  string|null  $key
     * @return mixed
     */
    public function json($key = null)
    {}

    /**
     * Get the decoded JSON body of the response as a collection.
     *
     * @param  string|null  $key
     * @return \Illuminate\Support\Collection
     */
    public function collect($key = null)
    {}

    /**
     * Assert that the response view equals the given value.
     *
     * @param  string  $value
     * @return $this
     */
    public function assertViewIs($value)
    {}

    /**
     * Assert that the response view has a given piece of bound data.
     *
     * @param  string|array  $key
     * @param  mixed  $value
     * @return $this
     */
    public function assertViewHas($key, $value = null)
    {}

    /**
     * Assert that the response view has a given list of bound data.
     *
     * @param  array  $bindings
     * @return $this
     */
    public function assertViewHasAll(array $bindings)
    {}

    /**
     * Get a piece of data from the original view.
     *
     * @param  string|null  $key
     * @return mixed
     */
    public function viewData($key = null)
    {}

    /**
     * Assert that the response view is missing a piece of bound data.
     *
     * @param  string  $key
     * @return $this
     */
    public function assertViewMissing($key)
    {}

    /**
     * Ensure that the response has a view as its original content.
     *
     * @return $this
     */
    protected function ensureResponseHasView()
    {}

    /**
     * Determine if the original response is a view.
     *
     * @return bool
     */
    protected function responseHasView()
    {}

    /**
     * Assert that the given keys do not have validation errors.
     *
     * @param  string|array|null  $keys
     * @param  string  $errorBag
     * @param  string  $responseKey
     * @return $this
     */
    public function assertValid($keys = null, $errorBag = 'default', $responseKey = 'errors')
    {}

    /**
     * Assert that the response has the given validation errors.
     *
     * @param  string|array|null  $errors
     * @param  string  $errorBag
     * @param  string  $responseKey
     * @return $this
     */
    public function assertInvalid($errors = null,
                                  $errorBag = 'default',
                                  $responseKey = 'errors')
    {}

    /**
     * Assert that the response has the given validation errors but does not have any other validation errors.
     *
     * @param  string|array|null  $errors
     * @param  string  $errorBag
     * @param  string  $responseKey
     * @return $this
     */
    public function assertOnlyInvalid($errors = null, $errorBag = 'default', $responseKey = 'errors')
    {}

    /**
     * Assert that the session has a given value.
     *
     * @param  string|array  $key
     * @param  mixed  $value
     * @return $this
     */
    public function assertSessionHas($key, $value = null)
    {}

    /**
     * Assert that the session has a given list of values.
     *
     * @param  array  $bindings
     * @return $this
     */
    public function assertSessionHasAll(array $bindings)
    {}

    /**
     * Assert that the session has a given value in the flashed input array.
     *
     * @param  string|array  $key
     * @param  mixed  $value
     * @return $this
     */
    public function assertSessionHasInput($key, $value = null)
    {}

    /**
     * Assert that the session is missing a given key in the flashed input array.
     *
     * @param  string|array  $key
     * @return $this
     */
    public function assertSessionMissingInput($key)
    {}

    /**
     * Assert that the session has the given errors.
     *
     * @param  string|array  $keys
     * @param  mixed  $format
     * @param  string  $errorBag
     * @return $this
     */
    public function assertSessionHasErrors($keys = [], $format = null, $errorBag = 'default')
    {}

    /**
     * Assert that the session is missing the given errors.
     *
     * @param  string|array  $keys
     * @param  string|null  $format
     * @param  string  $errorBag
     * @return $this
     */
    public function assertSessionDoesntHaveErrors($keys = [], $format = null, $errorBag = 'default')
    {}

    /**
     * Assert that the session has no errors.
     *
     * @return $this
     *
     * @throws \JsonException
     */
    public function assertSessionHasNoErrors()
    {}

    /**
     * Assert that the session has the given errors.
     *
     * @param  string  $errorBag
     * @param  string|array  $keys
     * @param  mixed  $format
     * @return $this
     */
    public function assertSessionHasErrorsIn($errorBag, $keys = [], $format = null)
    {}

    /**
     * Assert that the session does not have a given key.
     *
     * @param  string|array  $key
     * @param  mixed  $value
     * @return $this
     */
    public function assertSessionMissing($key, $value = null)
    {}

    /**
     * Get the current session store.
     *
     * @return \Illuminate\Session\Store
     */
    protected function session()
    {}

    /**
     * Dump the headers from the response and end the script.
     *
     * @return never
     */
    public function ddHeaders()
    {}

    /**
     * Dump the body of the response and end the script.
     *
     * @param  string|null  $key
     * @return never
     */
    public function ddBody($key = null)
    {}

    /**
     * Dump the JSON payload from the response and end the script.
     *
     * @param  string|null  $key
     * @return never
     */
    public function ddJson($key = null)
    {}

    /**
     * Dump the session from the response and end the script.
     *
     * @param  string|array  $keys
     * @return never
     */
    public function ddSession($keys = [])
    {}

    /**
     * Dump the content from the response.
     *
     * @param  string|null  $key
     * @return $this
     */
    public function dump($key = null)
    {}

    /**
     * Dump the headers from the response.
     *
     * @return $this
     */
    public function dumpHeaders()
    {}

    /**
     * Dump the session from the response.
     *
     * @param  string|array  $keys
     * @return $this
     */
    public function dumpSession($keys = [])
    {}

    /**
     * Get the streamed content from the response.
     *
     * @return string
     */
    public function streamedContent()
    {}

    /**
     * Set the previous exceptions on the response.
     *
     * @param  \Illuminate\Support\Collection  $exceptions
     * @return $this
     */
    public function withExceptions(Collection $exceptions)
    {}

    /**
     * Dynamically access base response parameters.
     *
     * @param  string  $key
     * @return mixed
     */
    public function __get($key)
    {}

    /**
     * Proxy isset() checks to the underlying base response.
     *
     * @param  string  $key
     * @return bool
     */
    public function __isset($key)
    {}

    /**
     * Determine if the given offset exists.
     *
     * @param  string  $offset
     * @return bool
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get the value for a given offset.
     *
     * @param  string  $offset
     * @return mixed
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set the value at the given offset.
     *
     * @param  string  $offset
     * @param  mixed  $value
     * @return void
     *
     * @throws \LogicException
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Unset the value at the given offset.
     *
     * @param  string  $offset
     * @return void
     *
     * @throws \LogicException
     */
    public function offsetUnset($offset): void
    {}

    /**
     * Handle dynamic calls into macros or pass missing methods to the base response.
     *
     * @param  string  $method
     * @param  array  $args
     * @return mixed
     */
    public function __call($method, $args)
    {}
}
