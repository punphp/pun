<?php

namespace Illuminate\Testing\Concerns;

use Illuminate\Testing\Assert as PHPUnit;

trait AssertsStatusCodes
{
    /**
     * Assert that the response has a 200 "OK" status code.
     *
     * @return $this
     */
    public function assertOk()
    {}

    /**
     * Assert that the response has a 201 "Created" status code.
     *
     * @return $this
     */
    public function assertCreated()
    {}

    /**
     * Assert that the response has a 202 "Accepted" status code.
     *
     * @return $this
     */
    public function assertAccepted()
    {}

    /**
     * Assert that the response has the given status code and no content.
     *
     * @param  int  $status
     * @return $this
     */
    public function assertNoContent($status = 204)
    {}

    /**
     * Assert that the response has a 301 "Moved Permanently" status code.
     *
     * @return $this
     */
    public function assertMovedPermanently()
    {}

    /**
     * Assert that the response has a 302 "Found" status code.
     *
     * @return $this
     */
    public function assertFound()
    {}

    /**
     * Assert that the response has a 304 "Not Modified" status code.
     *
     * @return $this
     */
    public function assertNotModified()
    {}

    /**
     * Assert that the response has a 307 "Temporary Redirect" status code.
     *
     * @return $this
     */
    public function assertTemporaryRedirect()
    {}

    /**
     * Assert that the response has a 308 "Permanent Redirect" status code.
     *
     * @return $this
     */
    public function assertPermanentRedirect()
    {}

    /**
     * Assert that the response has a 400 "Bad Request" status code.
     *
     * @return $this
     */
    public function assertBadRequest()
    {}

    /**
     * Assert that the response has a 401 "Unauthorized" status code.
     *
     * @return $this
     */
    public function assertUnauthorized()
    {}

    /**
     * Assert that the response has a 402 "Payment Required" status code.
     *
     * @return $this
     */
    public function assertPaymentRequired()
    {}

    /**
     * Assert that the response has a 403 "Forbidden" status code.
     *
     * @return $this
     */
    public function assertForbidden()
    {}

    /**
     * Assert that the response has a 404 "Not Found" status code.
     *
     * @return $this
     */
    public function assertNotFound()
    {}

    /**
     * Assert that the response has a 405 "Method Not Allowed" status code.
     *
     * @return $this
     */
    public function assertMethodNotAllowed()
    {}

    /**
     * Assert that the response has a 406 "Not Acceptable" status code.
     *
     * @return $this
     */
    public function assertNotAcceptable()
    {}

    /**
     * Assert that the response has a 408 "Request Timeout" status code.
     *
     * @return $this
     */
    public function assertRequestTimeout()
    {}

    /**
     * Assert that the response has a 409 "Conflict" status code.
     *
     * @return $this
     */
    public function assertConflict()
    {}

    /**
     * Assert that the response has a 410 "Gone" status code.
     *
     * @return $this
     */
    public function assertGone()
    {}

    /**
     * Assert that the response has a 415 "Unsupported Media Type" status code.
     *
     * @return $this
     */
    public function assertUnsupportedMediaType()
    {}

    /**
     * Assert that the response has a 422 "Unprocessable Content" status code.
     *
     * @return $this
     */
    public function assertUnprocessable()
    {}

    /**
     * Assert that the response has a 424 "Failed Dependency" status code.
     *
     * @return $this
     */
    public function assertFailedDependency()
    {}

    /**
     * Assert that the response has a 429 "Too Many Requests" status code.
     *
     * @return $this
     */
    public function assertTooManyRequests()
    {}

    /**
     * Assert that the response has a 500 "Internal Server Error" status code.
     *
     * @return $this
     */
    public function assertInternalServerError()
    {}

    /**
     * Assert that the response has a 503 "Service Unavailable" status code.
     *
     * @return $this
     */
    public function assertServiceUnavailable()
    {}
}
