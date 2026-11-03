<?php

namespace Illuminate\Foundation\Exceptions;

use Closure;
use Exception;
use Illuminate\Auth\Access\AuthorizationException;
use Illuminate\Auth\AuthenticationException;
use Illuminate\Cache\RateLimiter;
use Illuminate\Cache\RateLimiting\Limit;
use Illuminate\Cache\RateLimiting\Unlimited;
use Illuminate\Console\View\Components\BulletList;
use Illuminate\Console\View\Components\Error;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Debug\ExceptionHandler as ExceptionHandlerContract;
use Illuminate\Contracts\Debug\ShouldntReport;
use Illuminate\Contracts\Foundation\ExceptionRenderer;
use Illuminate\Contracts\Support\Responsable;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Database\RecordNotFoundException;
use Illuminate\Database\RecordsNotFoundException;
use Illuminate\Foundation\Exceptions\Renderer\Renderer;
use Illuminate\Http\Exceptions\HttpResponseException;
use Illuminate\Http\Exceptions\OriginMismatchException;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Response;
use Illuminate\Routing\Exceptions\BackedEnumCaseNotFoundException;
use Illuminate\Routing\Router;
use Illuminate\Session\TokenMismatchException;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Auth;
use Illuminate\Support\Lottery;
use Illuminate\Support\Reflector;
use Illuminate\Support\Stringable;
use Illuminate\Support\Traits\ReflectsClosures;
use Illuminate\Support\ViewErrorBag;
use Illuminate\Validation\ValidationException;
use InvalidArgumentException;
use Psr\Log\LoggerInterface;
use Psr\Log\LogLevel;
use Symfony\Component\Console\Application as ConsoleApplication;
use Symfony\Component\Console\Exception\CommandNotFoundException;
use Symfony\Component\ErrorHandler\ErrorRenderer\HtmlErrorRenderer;
use Symfony\Component\HttpFoundation\Exception\RequestExceptionInterface;
use Symfony\Component\HttpFoundation\RedirectResponse as SymfonyRedirectResponse;
use Symfony\Component\HttpFoundation\Response as SymfonyResponse;
use Symfony\Component\HttpKernel\Exception\AccessDeniedHttpException;
use Symfony\Component\HttpKernel\Exception\BadRequestHttpException;
use Symfony\Component\HttpKernel\Exception\HttpException;
use Symfony\Component\HttpKernel\Exception\HttpExceptionInterface;
use Symfony\Component\HttpKernel\Exception\NotFoundHttpException;
use Throwable;
use WeakMap;

class Handler implements ExceptionHandlerContract
{
    use ReflectsClosures;

    /**
     * The container implementation.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $container;

    /**
     * A list of the exception types that are not reported.
     *
     * @var array<int, class-string<\Throwable>>
     */
    protected $dontReport = [];

    /**
     * The callbacks that inspect exceptions to determine if they should be reported.
     *
     * @var array
     */
    protected $dontReportCallbacks = [];

    /**
     * A list of the exception types that should stop job retries.
     *
     * @var array<int, class-string<\Throwable>>
     */
    protected $dontRetry = [];

    /**
     * The callbacks that inspect exceptions to determine if they should stop job retries.
     *
     * @var array
     */
    protected $dontRetryCallbacks = [];

    /**
     * The callbacks that should be used during reporting.
     *
     * @var \Illuminate\Foundation\Exceptions\ReportableHandler[]
     */
    protected $reportCallbacks = [];

    /**
     * A map of exceptions with their corresponding custom log levels.
     *
     * @var array<class-string<\Throwable>, \Psr\Log\LogLevel::*>
     */
    protected $levels = [];

    /**
     * The callbacks that should be used to throttle reportable exceptions.
     *
     * @var array
     */
    protected $throttleCallbacks = [];

    /**
     * The callbacks that should be used to build exception context data.
     *
     * @var array
     */
    protected $contextCallbacks = [];

    /**
     * The exception currently being reported.
     *
     * @var \Throwable|null
     */
    protected ?Throwable $currentlyReporting = null;

    /**
     * The callbacks that should be used during rendering.
     *
     * @var \Closure[]
     */
    protected $renderCallbacks = [];

    /**
     * The callback that determines if the exception handler response should be JSON.
     *
     * @var callable|null
     */
    protected $shouldRenderJsonWhenCallback;

    /**
     * The callback that prepares responses to be returned to the browser.
     *
     * @var callable|null
     */
    protected $finalizeResponseCallback;

    /**
     * The registered exception mappings.
     *
     * @var array<string, \Closure>
     */
    protected $exceptionMap = [];

    /**
     * Indicates that throttled keys should be hashed.
     *
     * @var bool
     */
    protected $hashThrottleKeys = true;

    /**
     * A list of the internal exception types that should not be reported.
     *
     * @var array<int, class-string<\Throwable>>
     */
    protected $internalDontReport = [
        AuthenticationException::class,
        AuthorizationException::class,
        BackedEnumCaseNotFoundException::class,
        HttpException::class,
        HttpResponseException::class,
        ModelNotFoundException::class,
        OriginMismatchException::class,
        RecordNotFoundException::class,
        RecordsNotFoundException::class,
        RequestExceptionInterface::class,
        TokenMismatchException::class,
        ValidationException::class,
    ];

    /**
     * A list of the inputs that are never flashed for validation exceptions.
     *
     * @var array<int, string>
     */
    protected $dontFlash = [
        'current_password',
        'password',
        'password_confirmation',
    ];

    /**
     * Indicates that an exception instance should only be reported once.
     *
     * @var bool
     */
    protected $withoutDuplicates = false;

    /**
     * The already reported exception map.
     *
     * @var \WeakMap
     */
    protected $reportedExceptionMap;

    /**
     * Create a new exception handler instance.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     */
    public function __construct(Container $container)
    {}

    /**
     * Register the exception handling callbacks for the application.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Register a reportable callback.
     *
     * @param  callable  $reportUsing
     * @return \Illuminate\Foundation\Exceptions\ReportableHandler
     */
    public function reportable(callable $reportUsing)
    {}

    /**
     * Register a renderable callback.
     *
     * @param  callable  $renderUsing
     * @return $this
     */
    public function renderable(callable $renderUsing)
    {}

    /**
     * Register a new exception mapping.
     *
     * @param  \Closure|string  $from
     * @param  \Closure|string|null  $to
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function map($from, $to = null)
    {}

    /**
     * Indicate that the given exception type should not be reported.
     *
     * Alias of "ignore".
     *
     * @param  array|string  $exceptions
     * @return $this
     */
    public function dontReport(array|string $exceptions)
    {}

    /**
     * Register a callback to determine if an exception should not be reported.
     *
     * @param  (callable(\Throwable): bool)  $dontReportWhen
     * @return $this
     */
    public function dontReportWhen(callable $dontReportWhen)
    {}

    /**
     * Indicate that the given exception type should not be reported.
     *
     * @param  array|string  $exceptions
     * @return $this
     */
    public function ignore(array|string $exceptions)
    {}

    /**
     * Indicate that the given exception type should stop job retries.
     *
     * @param  array|string  $exceptions
     * @return $this
     */
    public function dontRetry(array|string $exceptions)
    {}

    /**
     * Register a callback to determine if an exception should stop job retries.
     *
     * @param  (callable(\Throwable): bool)  $dontRetryWhen
     * @return $this
     */
    public function dontRetryWhen(callable $dontRetryWhen)
    {}

    /**
     * Determine if the exception should stop job retries.
     *
     * @param  \Throwable  $e
     * @return bool
     */
    public function shouldStopRetries(Throwable $e)
    {}

    /**
     * Indicate that the given attributes should never be flashed to the session on validation errors.
     *
     * @param  array|string  $attributes
     * @return $this
     */
    public function dontFlash(array|string $attributes)
    {}

    /**
     * Set the log level for the given exception type.
     *
     * @param  class-string<\Throwable>  $type
     * @param  \Psr\Log\LogLevel::*  $level
     * @return $this
     */
    public function level($type, $level)
    {}

    /**
     * Report or log an exception.
     *
     * @param  \Throwable  $e
     * @return void
     *
     * @throws \Throwable
     */
    public function report(Throwable $e)
    {}

    /**
     * Reports error based on report method on exception or to logger.
     *
     * @param  \Throwable  $e
     * @return void
     *
     * @throws \Throwable
     */
    protected function reportThrowable(Throwable $e): void
    {}

    /**
     * Determine if a given exception is being reported.
     */
    public function isReporting(Throwable $e): bool
    {}

    /**
     * Determine if the exception should be reported.
     *
     * @param  \Throwable  $e
     * @return bool
     */
    public function shouldReport(Throwable $e)
    {}

    /**
     * Determine if the exception is in the "do not report" list.
     *
     * @param  \Throwable  $e
     * @return bool
     */
    protected function shouldntReport(Throwable $e)
    {}

    /**
     * Throttle the given exception.
     *
     * @param  \Throwable  $e
     * @return \Illuminate\Support\Lottery|\Illuminate\Cache\RateLimiting\Limit|null
     */
    protected function throttle(Throwable $e)
    {}

    /**
     * Specify the callback that should be used to throttle reportable exceptions.
     *
     * @param  callable  $throttleUsing
     * @return $this
     */
    public function throttleUsing(callable $throttleUsing)
    {}

    /**
     * Remove the given exception class from the list of exceptions that should be ignored.
     *
     * @param  array|string  $exceptions
     * @return $this
     */
    public function stopIgnoring(array|string $exceptions)
    {}

    /**
     * Create the context array for logging the given exception.
     *
     * @param  \Throwable  $e
     * @return array
     */
    public function contextForException(Throwable $e)
    {}

    /**
     * Create the context array for logging the given exception.
     *
     * @param  \Throwable  $e
     * @return array
     */
    protected function buildExceptionContext(Throwable $e)
    {}

    /**
     * Creates the context for an exception.
     *
     * @return array<array-key, mixed>
     */
    public function buildContextForException(Throwable $e)
    {}

    /**
     * Get the default exception context variables for logging.
     *
     * @param  \Throwable  $e
     * @return array
     */
    protected function exceptionContext(Throwable $e)
    {}

    /**
     * Get the default context variables for logging.
     *
     * @return array
     */
    protected function context()
    {}

    /**
     * Register a closure that should be used to build exception context data.
     *
     * @param  \Closure  $contextCallback
     * @return $this
     */
    public function buildContextUsing(Closure $contextCallback)
    {}

    /**
     * Render an exception into an HTTP response.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Throwable  $e
     * @return \Symfony\Component\HttpFoundation\Response
     *
     * @throws \Throwable
     */
    public function render($request, Throwable $e)
    {}

    /**
     * Prepare the final, rendered response to be returned to the browser.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Symfony\Component\HttpFoundation\Response  $response
     * @param  \Throwable  $e
     * @return \Symfony\Component\HttpFoundation\Response
     */
    protected function finalizeRenderedResponse($request, $response, Throwable $e)
    {}

    /**
     * Prepare the final, rendered response for an exception using the given callback.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function respondUsing($callback)
    {}

    /**
     * Prepare exception for rendering.
     *
     * @param  \Throwable  $e
     * @return \Throwable
     */
    protected function prepareException(Throwable $e)
    {}

    /**
     * Map the exception using a registered mapper if possible.
     *
     * @param  \Throwable  $e
     * @return \Throwable
     */
    protected function mapException(Throwable $e)
    {}

    /**
     * Try to render a response from request and exception via render callbacks.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Throwable  $e
     * @return mixed
     *
     * @throws \ReflectionException
     */
    protected function renderViaCallbacks($request, Throwable $e)
    {}

    /**
     * Render a default exception response if any.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Throwable  $e
     * @return \Illuminate\Http\Response|\Illuminate\Http\JsonResponse|\Illuminate\Http\RedirectResponse
     */
    protected function renderExceptionResponse($request, Throwable $e)
    {}

    /**
     * Convert an authentication exception into a response.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Illuminate\Auth\AuthenticationException  $exception
     * @return \Illuminate\Http\Response|\Illuminate\Http\JsonResponse|\Illuminate\Http\RedirectResponse
     */
    protected function unauthenticated($request, AuthenticationException $exception)
    {}

    /**
     * Create a response object from the given validation exception.
     *
     * @param  \Illuminate\Validation\ValidationException  $e
     * @param  \Illuminate\Http\Request  $request
     * @return \Symfony\Component\HttpFoundation\Response
     */
    protected function convertValidationExceptionToResponse(ValidationException $e, $request)
    {}

    /**
     * Convert a validation exception into a response.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Illuminate\Validation\ValidationException  $exception
     * @return \Illuminate\Http\Response|\Illuminate\Http\JsonResponse|\Illuminate\Http\RedirectResponse
     */
    protected function invalid($request, ValidationException $exception)
    {}

    /**
     * Convert a validation exception into a JSON response.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Illuminate\Validation\ValidationException  $exception
     * @return \Illuminate\Http\JsonResponse
     */
    protected function invalidJson($request, ValidationException $exception)
    {}

    /**
     * Determine if the exception handler response should be JSON.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Throwable  $e
     * @return bool
     */
    protected function shouldReturnJson($request, Throwable $e)
    {}

    /**
     * Register the callable that determines if the exception handler response should be JSON.
     *
     * @param  callable(\Illuminate\Http\Request $request, \Throwable): bool  $callback
     * @return $this
     */
    public function shouldRenderJsonWhen($callback)
    {}

    /**
     * Prepare a response for the given exception.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Throwable  $e
     * @return \Illuminate\Http\Response|\Illuminate\Http\JsonResponse|\Illuminate\Http\RedirectResponse
     */
    protected function prepareResponse($request, Throwable $e)
    {}

    /**
     * Create a Symfony response for the given exception.
     *
     * @param  \Throwable  $e
     * @return \Symfony\Component\HttpFoundation\Response
     */
    protected function convertExceptionToResponse(Throwable $e)
    {}

    /**
     * Get the response content for the given exception.
     *
     * @param  \Throwable  $e
     * @return string
     */
    protected function renderExceptionContent(Throwable $e)
    {}

    /**
     * Render an exception to a string using the registered `ExceptionRenderer`.
     *
     * @param  \Throwable  $e
     * @return string
     */
    protected function renderExceptionWithCustomRenderer(Throwable $e)
    {}

    /**
     * Render an exception to a string using Symfony.
     *
     * @param  \Throwable  $e
     * @param  bool  $debug
     * @return string
     */
    protected function renderExceptionWithSymfony(Throwable $e, $debug)
    {}

    /**
     * Render the given HttpException.
     *
     * @param  \Symfony\Component\HttpKernel\Exception\HttpExceptionInterface  $e
     * @return \Symfony\Component\HttpFoundation\Response
     *
     * @throws \Throwable
     */
    protected function renderHttpException(HttpExceptionInterface $e)
    {}

    /**
     * Register the error template hint paths.
     *
     * @return void
     */
    protected function registerErrorViewPaths()
    {}

    /**
     * Get the view used to render HTTP exceptions.
     *
     * @param  \Symfony\Component\HttpKernel\Exception\HttpExceptionInterface  $e
     * @return string|null
     */
    protected function getHttpExceptionView(HttpExceptionInterface $e)
    {}

    /**
     * Map the given exception into an Illuminate response.
     *
     * @param  \Symfony\Component\HttpFoundation\Response  $response
     * @param  \Throwable  $e
     * @return \Illuminate\Http\Response|\Illuminate\Http\RedirectResponse
     */
    protected function toIlluminateResponse($response, Throwable $e)
    {}

    /**
     * Prepare a JSON response for the given exception.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Throwable  $e
     * @return \Illuminate\Http\JsonResponse
     */
    protected function prepareJsonResponse($request, Throwable $e)
    {}

    /**
     * Convert the given exception to an array.
     *
     * @param  \Throwable  $e
     * @return array
     */
    protected function convertExceptionToArray(Throwable $e)
    {}

    /**
     * Render an exception to the console.
     *
     * @param  \Symfony\Component\Console\Output\OutputInterface  $output
     * @param  \Throwable  $e
     * @return void
     *
     * @internal This method is not meant to be used or overwritten outside the framework.
     */
    public function renderForConsole($output, Throwable $e)
    {}

    /**
     * Do not report duplicate exceptions.
     *
     * @return $this
     */
    public function dontReportDuplicates()
    {}

    /**
     * Determine if the given exception is an HTTP exception.
     *
     * @param  \Throwable  $e
     * @return bool
     */
    protected function isHttpException(Throwable $e)
    {}

    /**
     * Map the exception to a log level.
     *
     * @param  \Throwable  $e
     * @return \Psr\Log\LogLevel::*
     */
    protected function mapLogLevel(Throwable $e)
    {}

    /**
     * Create a new logger instance.
     *
     * @return \Psr\Log\LoggerInterface
     */
    protected function newLogger()
    {}
}
