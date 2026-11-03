<?php

namespace Illuminate\Http\Client\Concerns;

trait DeterminesStatusCode
{
    /**
     * Determine if the response code was 200 "OK" response.
     *
     * @return bool
     */
    public function ok()
    {}

    /**
     * Determine if the response code was 201 "Created" response.
     *
     * @return bool
     */
    public function created()
    {}

    /**
     * Determine if the response code was 202 "Accepted" response.
     *
     * @return bool
     */
    public function accepted()
    {}

    /**
     * Determine if the response code was the given status code and the body has no content.
     *
     * @param  int  $status
     * @return bool
     */
    public function noContent($status = 204)
    {}

    /**
     * Determine if the response code was a 301 "Moved Permanently".
     *
     * @return bool
     */
    public function movedPermanently()
    {}

    /**
     * Determine if the response code was a 302 "Found" response.
     *
     * @return bool
     */
    public function found()
    {}

    /**
     * Determine if the response code was a 304 "Not Modified" response.
     *
     * @return bool
     */
    public function notModified()
    {}

    /**
     * Determine if the response was a 400 "Bad Request" response.
     *
     * @return bool
     */
    public function badRequest()
    {}

    /**
     * Determine if the response was a 401 "Unauthorized" response.
     *
     * @return bool
     */
    public function unauthorized()
    {}

    /**
     * Determine if the response was a 402 "Payment Required" response.
     *
     * @return bool
     */
    public function paymentRequired()
    {}

    /**
     * Determine if the response was a 403 "Forbidden" response.
     *
     * @return bool
     */
    public function forbidden()
    {}

    /**
     * Determine if the response was a 404 "Not Found" response.
     *
     * @return bool
     */
    public function notFound()
    {}

    /**
     * Determine if the response was a 408 "Request Timeout" response.
     *
     * @return bool
     */
    public function requestTimeout()
    {}

    /**
     * Determine if the response was a 409 "Conflict" response.
     *
     * @return bool
     */
    public function conflict()
    {}

    /**
     * Determine if the response was a 422 "Unprocessable Content" response.
     *
     * @return bool
     */
    public function unprocessableContent()
    {}

    /**
     * Determine if the response was a 422 "Unprocessable Content" response.
     *
     * @return bool
     */
    public function unprocessableEntity()
    {}

    /**
     * Determine if the response was a 429 "Too Many Requests" response.
     *
     * @return bool
     */
    public function tooManyRequests()
    {}
}
