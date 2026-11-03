<?php

namespace Illuminate\Support\Testing\Fakes;

use Closure;
use Illuminate\Contracts\Debug\ExceptionHandler;
use Illuminate\Foundation\Testing\Concerns\WithoutExceptionHandlingHandler;
use Illuminate\Support\Collection;
use Illuminate\Support\Traits\ForwardsCalls;
use Illuminate\Support\Traits\ReflectsClosures;
use Illuminate\Testing\Assert;
use PHPUnit\Framework\Assert as PHPUnit;
use PHPUnit\Framework\ExpectationFailedException;
use Throwable;

/**
 * @mixin \Illuminate\Foundation\Exceptions\Handler
 */
class ExceptionHandlerFake implements ExceptionHandler, Fake
{
    use ForwardsCalls, ReflectsClosures;

    /**
     * All of the exceptions that have been reported.
     *
     * @var list<\Throwable>
     */
    protected $reported = [];

    /**
     * If the fake should throw exceptions when they are reported.
     *
     * @var bool
     */
    protected $throwOnReport = false;

    /**
     * Create a new exception handler fake.
     *
     * @param  \Illuminate\Contracts\Debug\ExceptionHandler  $handler
     * @param  list<class-string<\Throwable>>  $exceptions
     */
    public function __construct(
        protected ExceptionHandler $handler,
        protected array $exceptions = [],
    ) {}

    /**
     * Get the underlying handler implementation.
     *
     * @return \Illuminate\Contracts\Debug\ExceptionHandler
     */
    public function handler()
    {}

    /**
     * Assert if an exception of the given type has been reported.
     *
     * @param  (\Closure(\Throwable): bool)|class-string<\Throwable>  $exception
     * @return void
     */
    public function assertReported(Closure|string $exception)
    {}

    /**
     * Assert the number of exceptions that have been reported.
     *
     * @param  int  $count
     * @return void
     */
    public function assertReportedCount(int $count)
    {}

    /**
     * Assert if an exception of the given type has not been reported.
     *
     * @param  (\Closure(\Throwable): bool)|class-string<\Throwable>  $exception
     * @return void
     *
     * @throws \PHPUnit\Framework\ExpectationFailedException
     */
    public function assertNotReported(Closure|string $exception)
    {}

    /**
     * Assert nothing has been reported.
     *
     * @return void
     */
    public function assertNothingReported()
    {}

    /**
     * Report or log an exception.
     *
     * @param  \Throwable  $e
     * @return void
     *
     * @throws \Throwable
     */
    public function report($e)
    {}

    /**
     * Determine if the given exception is faked.
     *
     * @param  \Throwable  $e
     * @return bool
     */
    protected function isFakedException(Throwable $e)
    {}

    /**
     * Determine if the exception should be reported.
     *
     * @param  \Throwable  $e
     * @return bool
     */
    public function shouldReport($e)
    {}

    /**
     * Determine if the handler is running without exception handling.
     *
     * @return bool
     */
    protected function runningWithoutExceptionHandling()
    {}

    /**
     * Render an exception into an HTTP response.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Throwable  $e
     * @return \Symfony\Component\HttpFoundation\Response
     */
    public function render($request, $e)
    {}

    /**
     * Render an exception to the console.
     *
     * @param  \Symfony\Component\Console\Output\OutputInterface  $output
     * @param  \Throwable  $e
     * @return void
     */
    public function renderForConsole($output, Throwable $e)
    {}

    /**
     * Throw exceptions when they are reported.
     *
     * @return $this
     */
    public function throwOnReport()
    {}

    /**
     * Throw the first reported exception.
     *
     * @return $this
     *
     * @throws \Throwable
     */
    public function throwFirstReported()
    {}

    /**
     * Get the exceptions that have been reported.
     *
     * @return list<\Throwable>
     */
    public function reported()
    {}

    /**
     * Set the "original" handler that should be used by the fake.
     *
     * @param  \Illuminate\Contracts\Debug\ExceptionHandler  $handler
     * @return $this
     */
    public function setHandler(ExceptionHandler $handler)
    {}

    /**
     * Handle dynamic method calls to the handler.
     *
     * @param  string  $method
     * @param  array<string, mixed>  $parameters
     * @return mixed
     */
    public function __call(string $method, array $parameters)
    {}
}
