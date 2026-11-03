<?php

namespace Illuminate\Testing;

use Illuminate\Http\RedirectResponse;
use Illuminate\Support\Arr;
use Illuminate\Support\ViewErrorBag;
use PHPUnit\Framework\ExpectationFailedException;
use ReflectionProperty;

/**
 * @internal
 *
 * @mixin Assert
 */
class TestResponseAssert
{
    /**
     * Create a new TestResponse assertion helper.
     */
    private function __construct(protected TestResponse $response)
    {}

    /**
     * Create a new TestResponse assertion helper.
     */
    public static function withResponse(TestResponse $response): static
    {}

    /**
     * Pass method calls to the Assert class and decorate the exception message.
     *
     * @param  string  $name
     * @param  array  $arguments
     * @return void
     *
     * @throws \PHPUnit\Framework\ExpectationFailedException
     */
    public function __call($name, $arguments)
    {}

    /**
     * Pass static method calls to the Assert class.
     *
     * @param  string  $name
     * @param  array  $arguments
     * @return void
     *
     * @throws \PHPUnit\Framework\ExpectationFailedException
     */
    public static function __callStatic($name, $arguments)
    {}

    /**
     * Inject additional context from the response into the exception message.
     *
     * @param  \PHPUnit\Framework\ExpectationFailedException  $exception
     * @return \PHPUnit\Framework\ExpectationFailedException
     */
    protected function injectResponseContext($exception)
    {}

    /**
     * Append an exception to the message of another exception.
     *
     * @param  \Throwable  $exceptionToAppend
     * @param  \PHPUnit\Framework\ExpectationFailedException  $exception
     * @return \PHPUnit\Framework\ExpectationFailedException
     */
    protected function appendExceptionToException($exceptionToAppend, $exception)
    {}

    /**
     * Append errors to an exception message.
     *
     * @param  array  $errors
     * @param  \PHPUnit\Framework\ExpectationFailedException  $exception
     * @param  bool  $json
     * @return \PHPUnit\Framework\ExpectationFailedException
     */
    protected function appendErrorsToException($errors, $exception, $json = false)
    {}

    /**
     * Append a message to an exception.
     *
     * @param  string  $message
     * @param  \PHPUnit\Framework\ExpectationFailedException  $exception
     * @return \PHPUnit\Framework\ExpectationFailedException
     */
    protected function appendMessageToException($message, $exception)
    {}
}
