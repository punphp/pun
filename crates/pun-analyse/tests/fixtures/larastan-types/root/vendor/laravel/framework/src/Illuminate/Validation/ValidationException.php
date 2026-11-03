<?php

namespace Illuminate\Validation;

use Exception;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Validator as ValidatorFacade;

class ValidationException extends Exception
{
    /**
     * The validator instance.
     *
     * @var \Illuminate\Contracts\Validation\Validator
     */
    public $validator;

    /**
     * The recommended response to send to the client.
     *
     * @var \Symfony\Component\HttpFoundation\Response|null
     */
    public $response;

    /**
     * The status code to use for the response.
     *
     * @var int
     */
    public $status = 422;

    /**
     * The name of the error bag.
     *
     * @var string
     */
    public $errorBag;

    /**
     * The path the client should be redirected to.
     *
     * @var string|null
     */
    public $redirectTo;

    /**
     * Create a new exception instance.
     *
     * @param  \Illuminate\Contracts\Validation\Validator  $validator
     * @param  \Symfony\Component\HttpFoundation\Response|null  $response
     * @param  string  $errorBag
     */
    public function __construct($validator, $response = null, $errorBag = 'default')
    {}

    /**
     * Create a new validation exception from a plain array of messages.
     *
     * @param  array  $messages
     * @return static
     */
    public static function withMessages(array $messages)
    {}

    /**
     * Create an error message summary from the validation errors.
     *
     * @param  \Illuminate\Contracts\Validation\Validator  $validator
     * @return string
     */
    protected static function summarize($validator)
    {}

    /**
     * Get all of the validation error messages.
     *
     * @return array
     */
    public function errors()
    {}

    /**
     * Set the HTTP status code to be used for the response.
     *
     * @param  int  $status
     * @return $this
     */
    public function status($status)
    {}

    /**
     * Set the error bag on the exception.
     *
     * @param  string  $errorBag
     * @return $this
     */
    public function errorBag($errorBag)
    {}

    /**
     * Set the URL to redirect to on a validation error.
     *
     * @param  string  $url
     * @return $this
     */
    public function redirectTo($url)
    {}

    /**
     * Get the underlying response instance.
     *
     * @return \Symfony\Component\HttpFoundation\Response|null
     */
    public function getResponse()
    {}
}
