<?php

namespace Illuminate\Foundation\Testing\Concerns;

use Closure;
use Illuminate\Contracts\Debug\ExceptionHandler;
use Illuminate\Support\Testing\Fakes\ExceptionHandlerFake;
use Illuminate\Support\Traits\ReflectsClosures;
use Illuminate\Testing\Assert;
use Illuminate\Validation\ValidationException;
use Symfony\Component\Console\Application as ConsoleApplication;
use Symfony\Component\HttpKernel\Exception\NotFoundHttpException;
use Throwable;

trait InteractsWithExceptionHandling
{
    use ReflectsClosures;

    /**
     * The original exception handler.
     *
     * @var \Illuminate\Contracts\Debug\ExceptionHandler|null
     */
    protected $originalExceptionHandler;

    /**
     * Restore exception handling.
     *
     * @return $this
     */
    protected function withExceptionHandling()
    {}

    /**
     * Only handle the given exceptions via the exception handler.
     *
     * @param  list<class-string<\Throwable>>  $exceptions
     * @return $this
     */
    protected function handleExceptions(array $exceptions)
    {}

    /**
     * Only handle validation exceptions via the exception handler.
     *
     * @return $this
     */
    protected function handleValidationExceptions()
    {}

    /**
     * Disable exception handling for the test.
     *
     * @param  list<class-string<\Throwable>>  $except
     * @return $this
     */
    protected function withoutExceptionHandling(array $except = [])
    {}

    /**
     * Assert that the given callback throws an exception with the given message when invoked.
     *
     * @param  \Closure  $test
     * @param  (\Closure(\Throwable): bool)|class-string<\Throwable>  $expectedClass
     * @param  string|null  $expectedMessage
     * @return $this
     */
    protected function assertThrows(Closure $test, string|Closure $expectedClass = Throwable::class, ?string $expectedMessage = null)
    {}

    /**
     * Assert that the given callback does not throw an exception.
     *
     * @param  \Closure  $test
     * @return $this
     */
    protected function assertDoesntThrow(Closure $test)
    {}
}
