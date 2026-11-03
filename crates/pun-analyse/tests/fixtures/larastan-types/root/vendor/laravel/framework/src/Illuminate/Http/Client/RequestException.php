<?php

namespace Illuminate\Http\Client;

use GuzzleHttp\Psr7\Message;

class RequestException extends HttpClientException
{
    /**
     * The response instance.
     *
     * @var \Illuminate\Http\Client\Response
     */
    public $response;

    /**
     * The current truncation length for the exception message.
     *
     * @var int|false|null
     */
    public $truncateExceptionsAt;

    /**
     * The global truncation length for the exception message.
     *
     * @var int|false
     */
    public static $truncateAt = 120;

    /**
     * Whether the response has been summarized in the message.
     *
     * @var bool
     */
    public $hasBeenSummarized = false;

    /**
     * Create a new exception instance.
     *
     * @param  \Illuminate\Http\Client\Response  $response
     * @param  int|false|null  $truncateExceptionsAt
     */
    public function __construct(Response $response, $truncateExceptionsAt = null)
    {}

    /**
     * Enable truncation of request exception messages.
     *
     * @return void
     */
    public static function truncate()
    {}

    /**
     * Set the truncation length for request exception messages.
     *
     * @param  int  $length
     * @return void
     */
    public static function truncateAt(int $length)
    {}

    /**
     * Disable truncation of request exception messages.
     *
     * @return void
     */
    public static function dontTruncate()
    {}

    /**
     * Prepare the exception message.
     *
     * @return bool
     */
    public function report()
    {}

    /**
     * Prepare the exception message.
     *
     * @param  \Illuminate\Http\Client\Response  $response
     * @return string
     */
    protected function prepareMessage(Response $response)
    {}
}
