<?php

namespace Illuminate\Foundation\Testing\Concerns;

use BackedEnum;
use Illuminate\Contracts\Http\Kernel as HttpKernel;
use Illuminate\Cookie\CookieValuePrefix;
use Illuminate\Http\Request;
use Illuminate\Support\Collection;
use Illuminate\Support\Uri;
use Illuminate\Testing\LoggedExceptionCollection;
use Illuminate\Testing\TestResponse;
use Symfony\Component\HttpFoundation\File\UploadedFile as SymfonyUploadedFile;
use Symfony\Component\HttpFoundation\Request as SymfonyRequest;

trait MakesHttpRequests
{
    /**
     * Additional headers for the request.
     *
     * @var array
     */
    protected $defaultHeaders = [];

    /**
     * Additional cookies for the request.
     *
     * @var array
     */
    protected $defaultCookies = [];

    /**
     * Additional cookies will not be encrypted for the request.
     *
     * @var array
     */
    protected $unencryptedCookies = [];

    /**
     * Additional server variables for the request.
     *
     * @var array
     */
    protected $serverVariables = [];

    /**
     * Indicates whether redirects should be followed.
     *
     * @var bool
     */
    protected $followRedirects = false;

    /**
     * Indicates whether cookies should be encrypted.
     *
     * @var bool
     */
    protected $encryptCookies = true;

    /**
     * Indicated whether JSON requests should be performed "with credentials" (cookies).
     *
     * @see https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/withCredentials
     *
     * @var bool
     */
    protected $withCredentials = false;

    /**
     * Define additional headers to be sent with the request.
     *
     * @param  array  $headers
     * @return $this
     */
    public function withHeaders(array $headers)
    {}

    /**
     * Add a header to be sent with the request.
     *
     * @param  string  $name
     * @param  string  $value
     * @return $this
     */
    public function withHeader(string $name, string $value)
    {}

    /**
     * Remove a header from the request.
     *
     * @param  string  $name
     * @return $this
     */
    public function withoutHeader(string $name)
    {}

    /**
     * Remove headers from the request.
     *
     * @param  array  $headers
     * @return $this
     */
    public function withoutHeaders(array $headers)
    {}

    /**
     * Add an authorization token for the request.
     *
     * @param  string  $token
     * @param  string  $type
     * @return $this
     */
    public function withToken(#[\SensitiveParameter] string $token, string $type = 'Bearer')
    {}

    /**
     * Add a basic authentication header to the request with the given credentials.
     *
     * @param  string  $username
     * @param  string  $password
     * @return $this
     */
    public function withBasicAuth(string $username, #[\SensitiveParameter] string $password)
    {}

    /**
     * Remove the authorization token from the request.
     *
     * @return $this
     */
    public function withoutToken()
    {}

    /**
     * Flush all the configured headers.
     *
     * @return $this
     */
    public function flushHeaders()
    {}

    /**
     * Define a set of server variables to be sent with the requests.
     *
     * @param  array  $server
     * @return $this
     */
    public function withServerVariables(array $server)
    {}

    /**
     * Disable middleware for the test.
     *
     * @param  string|array|null  $middleware
     * @return $this
     */
    public function withoutMiddleware($middleware = null)
    {}

    /**
     * Enable the given middleware for the test.
     *
     * @param  string|array|null  $middleware
     * @return $this
     */
    public function withMiddleware($middleware = null)
    {}

    /**
     * Define additional cookies to be sent with the request.
     *
     * @param  array  $cookies
     * @return $this
     */
    public function withCookies(array $cookies)
    {}

    /**
     * Add a cookie to be sent with the request.
     *
     * @param  string  $name
     * @param  string  $value
     * @return $this
     */
    public function withCookie(string $name, string $value)
    {}

    /**
     * Define additional cookies will not be encrypted before sending with the request.
     *
     * @param  array  $cookies
     * @return $this
     */
    public function withUnencryptedCookies(array $cookies)
    {}

    /**
     * Add a cookie will not be encrypted before sending with the request.
     *
     * @param  string  $name
     * @param  string  $value
     * @return $this
     */
    public function withUnencryptedCookie(string $name, string $value)
    {}

    /**
     * Automatically follow any redirects returned from the response.
     *
     * @return $this
     */
    public function followingRedirects()
    {}

    /**
     * Include cookies and authorization headers for JSON requests.
     *
     * @return $this
     */
    public function withCredentials()
    {}

    /**
     * Disable automatic encryption of cookie values.
     *
     * @return $this
     */
    public function disableCookieEncryption()
    {}

    /**
     * Set the referer header and previous URL session value from a given URL in order to simulate a previous request.
     *
     * @param  string  $url
     * @return $this
     */
    public function from(string $url)
    {}

    /**
     * Set the referer header and previous URL session value from a given route in order to simulate a previous request.
     *
     * @param  \BackedEnum|string  $name
     * @param  mixed  $parameters
     * @return $this
     */
    public function fromRoute(BackedEnum|string $name, $parameters = [])
    {}

    /**
     * Set the Precognition header to "true".
     *
     * @return $this
     */
    public function withPrecognition()
    {}

    /**
     * Visit the given URI with a GET request.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $headers
     * @return \Illuminate\Testing\TestResponse
     */
    public function get($uri, array $headers = [])
    {}

    /**
     * Visit the given URI with a GET request, expecting a JSON response.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $headers
     * @param  int  $options
     * @return \Illuminate\Testing\TestResponse
     */
    public function getJson($uri, array $headers = [], $options = 0)
    {}

    /**
     * Visit the given URI with a POST request.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @return \Illuminate\Testing\TestResponse
     */
    public function post($uri, array $data = [], array $headers = [])
    {}

    /**
     * Visit the given URI with a POST request, expecting a JSON response.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @param  int  $options
     * @return \Illuminate\Testing\TestResponse
     */
    public function postJson($uri, array $data = [], array $headers = [], $options = 0)
    {}

    /**
     * Visit the given URI with a PUT request.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @return \Illuminate\Testing\TestResponse
     */
    public function put($uri, array $data = [], array $headers = [])
    {}

    /**
     * Visit the given URI with a PUT request, expecting a JSON response.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @param  int  $options
     * @return \Illuminate\Testing\TestResponse
     */
    public function putJson($uri, array $data = [], array $headers = [], $options = 0)
    {}

    /**
     * Visit the given URI with a PATCH request.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @return \Illuminate\Testing\TestResponse
     */
    public function patch($uri, array $data = [], array $headers = [])
    {}

    /**
     * Visit the given URI with a PATCH request, expecting a JSON response.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @param  int  $options
     * @return \Illuminate\Testing\TestResponse
     */
    public function patchJson($uri, array $data = [], array $headers = [], $options = 0)
    {}

    /**
     * Visit the given URI with a DELETE request.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @return \Illuminate\Testing\TestResponse
     */
    public function delete($uri, array $data = [], array $headers = [])
    {}

    /**
     * Visit the given URI with a DELETE request, expecting a JSON response.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @param  int  $options
     * @return \Illuminate\Testing\TestResponse
     */
    public function deleteJson($uri, array $data = [], array $headers = [], $options = 0)
    {}

    /**
     * Visit the given URI with an OPTIONS request.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @return \Illuminate\Testing\TestResponse
     */
    public function options($uri, array $data = [], array $headers = [])
    {}

    /**
     * Visit the given URI with an OPTIONS request, expecting a JSON response.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @param  int  $options
     * @return \Illuminate\Testing\TestResponse
     */
    public function optionsJson($uri, array $data = [], array $headers = [], $options = 0)
    {}

    /**
     * Visit the given URI with a HEAD request.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $headers
     * @return \Illuminate\Testing\TestResponse
     */
    public function head($uri, array $headers = [])
    {}

    /**
     * Visit the given URI with a QUERY request.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @return \Illuminate\Testing\TestResponse
     */
    public function query($uri, array $data = [], array $headers = [])
    {}

    /**
     * Visit the given URI with a QUERY request, expecting a JSON response.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @param  int  $options
     * @return \Illuminate\Testing\TestResponse
     */
    public function queryJson($uri, array $data = [], array $headers = [], $options = 0)
    {}

    /**
     * Call the given URI with a JSON request.
     *
     * @param  string  $method
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $data
     * @param  array  $headers
     * @param  int  $options
     * @return \Illuminate\Testing\TestResponse
     */
    public function json($method, $uri, array $data = [], array $headers = [], $options = 0)
    {}

    /**
     * Call the given URI and return the Response.
     *
     * @param  string  $method
     * @param  \Illuminate\Support\Uri|string  $uri
     * @param  array  $parameters
     * @param  array  $cookies
     * @param  array  $files
     * @param  array  $server
     * @param  string|null  $content
     * @return \Illuminate\Testing\TestResponse
     */
    public function call($method, $uri, $parameters = [], $cookies = [], $files = [], $server = [], $content = null)
    {}

    /**
     * Turn the given URI into a fully-qualified URL.
     *
     * @param  \Illuminate\Support\Uri|string  $uri
     * @return string
     */
    protected function prepareUrlForRequest($uri)
    {}

    /**
     * Transform headers array to array of $_SERVER vars with HTTP_* format.
     *
     * @param  array  $headers
     * @return array
     */
    protected function transformHeadersToServerVars(array $headers)
    {}

    /**
     * Format the header name for the server array.
     *
     * @param  string  $name
     * @return string
     */
    protected function formatServerHeaderKey($name)
    {}

    /**
     * Extract the file uploads from the given data array.
     *
     * @param  array  $data
     * @return array
     */
    protected function extractFilesFromDataArray(&$data)
    {}

    /**
     * If enabled, encrypt cookie values for request.
     *
     * @return array
     */
    protected function prepareCookiesForRequest()
    {}

    /**
     * If enabled, add cookies for JSON requests.
     *
     * @return array
     */
    protected function prepareCookiesForJsonRequest()
    {}

    /**
     * Follow a redirect chain until a non-redirect is received.
     *
     * @param  \Illuminate\Http\Response|\Illuminate\Testing\TestResponse  $response
     * @return \Illuminate\Http\Response|\Illuminate\Testing\TestResponse
     */
    protected function followRedirects($response)
    {}

    /**
     * Create the request instance used for testing from the given Symfony request.
     *
     * @param  \Symfony\Component\HttpFoundation\Request  $symfonyRequest
     * @return \Illuminate\Http\Request
     */
    protected function createTestRequest($symfonyRequest)
    {}

    /**
     * Create the test response instance from the given response.
     *
     * @param  \Illuminate\Http\Response  $response
     * @param  \Illuminate\Http\Request  $request
     * @return \Illuminate\Testing\TestResponse
     */
    protected function createTestResponse($response, $request)
    {}
}
