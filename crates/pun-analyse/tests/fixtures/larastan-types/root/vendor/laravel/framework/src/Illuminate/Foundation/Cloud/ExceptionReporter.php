<?php

namespace Illuminate\Foundation\Cloud;

use Closure;
use DateTimeZone;
use Illuminate\Console\Application as ConsoleApplication;
use Illuminate\Console\Scheduling\CallbackEvent as CallbackTask;
use Illuminate\Console\Scheduling\Event as ScheduledTask;
use Illuminate\Contracts\Auth\Authenticatable;
use Illuminate\Contracts\Queue\Job;
use Illuminate\Foundation\Bootstrap\HandleExceptions;
use Illuminate\Foundation\Exceptions\Renderer\Mappers\BladeMapper;
use Illuminate\Log\Context\Repository as ContextRepository;
use Illuminate\Routing\Route;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\App;
use Illuminate\Support\Facades\Artisan;
use Illuminate\Support\Facades\Auth;
use Illuminate\Support\Facades\Config;
use Illuminate\Support\Facades\Context;
use Illuminate\Support\Facades\Date;
use Illuminate\Support\Facades\Exceptions;
use Illuminate\Support\Facades\Request;
use Illuminate\View\ViewException;
use Ramsey\Uuid\Uuid;
use ReflectionClass;
use ReflectionFunction;
use RuntimeException;
use Spatie\LaravelIgnition\Exceptions\ViewException as IgnitionViewException;
use Symfony\Component\Console\Exception\CommandNotFoundException;
use Symfony\Component\Console\Input\ArgvInput;
use Symfony\Component\Console\Input\Input as ConsoleInput;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Input\RawInputInterface;
use Symfony\Component\ErrorHandler\Error\FatalError;
use Symfony\Component\HttpFoundation\HeaderBag;
use Throwable;
use WeakMap;

class ExceptionReporter
{
    /**
     * Indicates the currently reporting exception is a view exception.
     */
    protected bool $reportingViewException = false;

    /**
     * The Blade mapper resolved for the currently reporting exception.
     */
    protected ?BladeMapper $bladeMapper = null;

    /**
     * The captured identifiers, keyed by exception.
     *
     * @var \WeakMap<\Throwable, string>
     */
    protected WeakMap $exceptionIds;

    /**
     * The time the Octane request was received.
     */
    protected ?float $octaneRequestReceivedAt = null;

    /**
     * The name of the currently running Artisan command.
     */
    protected ?string $currentlyRunningCommandName = null;

    /**
     * The trace ID for the current artisan command execution.
     */
    protected ?string $artisanCommandTraceId = null;

    /**
     * The console input of the currently running Artisan command.
     */
    protected ?ConsoleInput $currentConsoleInput = null;

    /**
     * The console input built from the process arguments. Fallback when console input is not yet set.
     */
    protected ?ArgvInput $currentFallbackArgvInput = null;

    /**
     * The currently running scheduled task.
     */
    protected ?ScheduledTask $currentlyRunningScheduledTask = null;

    /**
     * Indicates the currently running scheduled task has finished with its failure yet to be reported.
     */
    protected bool $scheduledTaskAwaitingFailureReport = false;

    /**
     * The currently executing job on the queue.
     */
    protected ?Job $currentlyProcessingJob = null;

    /**
     * The currently executing job attempt ID.
     */
    protected ?string $currentlyProcessingJobAttemptId = null;

    /**
     * The cached normalized queue names, keyed by "connection:queue".
     *
     * @var array<string, string>
     */
    protected array $normalizedQueues = [];

    /**
     * The cached queue configuration.
     *
     * @var array<string, mixed>|null
     */
    protected ?array $connectionConfig = null;

    /**
     * Proactively captured execution context, keyed by execution type.
     *
     * @var array{
     *    job: array<string, mixed>,
     *    scheduled_task: array<string, mixed>,
     * }
     */
    protected array $executionContext = [
        'job' => [],
        'scheduled_task' => [],
    ];

    /**
     * The memory reserved for reporting fatal errors.
     */
    protected ?string $reservedMemory;

    /**
     * Create a new Exception Reporter instance.
     *
     * @param  array{
     *    stop: bool,
     *    capture_request_payload: bool,
     *    redact_request_payload_fields: list<string>,
     *    redact_headers: list<string>,
     *    redact_command_input_fields: list<string>,
     * }  $config
     */
    public function __construct(
        protected Events $events,
        protected Closure $bladeMapperResolver,
        protected string $basePath,
        protected array $config,
    ) {}

    /**
     * Report the given exception.
     */
    public function __invoke(Throwable $e): ?bool
    {}

    /**
     * Retrieve the identifier for the given exception.
     */
    public function exceptionId(Throwable $e): string
    {}

    /**
     * Unwrap view exceptions and remember the original exception.
     */
    protected function unwrapViewException(Throwable $e): Throwable
    {}

    /**
     * Determine if the given exception is a view exception.
     */
    protected function isViewException(Throwable $e): bool
    {}

    /**
     * Create the exception payload.
     *
     * @return array<string, mixed>
     */
    protected function payload(Throwable $e): array
    {}

    /**
     * Retrieve the current timestamp.
     */
    protected function timestamp(): string
    {}

    /**
     * Retrieve the context for the given exception.
     */
    protected function exceptionContext(Throwable $e): ?object
    {}

    /**
     * Retrieve the current Laravel Context.
     */
    protected function laravelContext(): ?object
    {}

    /**
     * Retrieve the execution context.
     *
     * @return array<string, mixed>
     */
    protected function executionDetails(Throwable $e): array
    {}

    /**
     * Prepare for an incoming Octane request.
     */
    public function prepareForOctaneRequest(): void
    {}

    /**
     * Retrieve the request execution context.
     *
     * @return array<string, mixed>
     */
    protected function requestExecutionDetails(Throwable $e): array
    {}

    /**
     * Retrieve the request headers.
     *
     * @return array<string, list<string|null>>|null
     */
    protected function requestHeaders(): ?array
    {}

    /**
     * Remove the headers PHP derives from the Authorization header.
     */
    protected function removeSyntheticAuthorizationHeaders(HeaderBag $headers): void
    {}

    /**
     * Redact the configured sensitive headers.
     */
    protected function redactHeaders(HeaderBag $headers): void
    {}

    /**
     * Redact the given authorization header value, retaining the scheme.
     */
    protected function redactAuthorizationHeaderValue(string $value): string
    {}

    /**
     * Redact the given cookie header value, retaining the cookie names.
     */
    protected function redactCookieHeaderValue(string $value): string
    {}

    /**
     * Retrieve the requested URL.
     */
    protected function requestUrl(): string
    {}

    /**
     * Retrieve the route specific request execution context.
     *
     * @return array<string, mixed>|null
     */
    protected function requestRouteExecutionDetails(): ?array
    {}

    /**
     * Retrieve the request payload.
     *
     * @return array<array-key, mixed>|null
     */
    protected function requestPayload(Throwable $e): ?array
    {}

    /**
     * Redact the configured sensitive fields in the given payload.
     *
     * @param  array<array-key, mixed>  $payload
     * @return array<array-key, mixed>
     */
    protected function redactRequestPayload(array $payload): array
    {}

    /**
     * Determine if the given payload field should be redacted.
     */
    protected function shouldRedactRequestPayloadField(string $field, mixed $value): bool
    {}

    /**
     * Retrieve the parsed uploaded files.
     *
     * @return array<array-key, mixed>|null
     */
    protected function requestFiles(Throwable $e): ?array
    {}

    /**
     * Parse the given uploaded files into their reportable details.
     *
     * Values are uploaded files, or nested arrays of them.
     *
     * @param  array<array-key, mixed>  $files
     * @return array<array-key, mixed>
     */
    protected function parseRequestFiles(array $files): array
    {}

    /**
     * Prepare to process the given command.
     */
    public function prepareForCommand(string $name, InputInterface $input): void
    {}

    /**
     * Retrieve the console execution context.
     *
     * @return array<string, mixed>
     */
    protected function consoleCommandExecutionDetails(Throwable $e): array
    {}

    /**
     * Retrieve the console command execution trace ID.
     */
    protected function consoleCommandTraceId(): string
    {}

    /**
     * Retrieve the name of the currently running Artisan command.
     */
    protected function consoleCommandName(): ?string
    {}

    /**
     * Retrieve the class name of the currently running Artisan command.
     */
    protected function consoleCommandClass(): ?string
    {}

    /**
     * Retrieve the redacted command line.
     */
    protected function consoleCommandLine(): ?string
    {}

    /**
     * Retrieve a transformer for the given console argument.
     */
    protected function consoleArgumentTransformer(string $name): callable
    {}

    /**
     * Retrieve a transformer for the given console option.
     */
    protected function consoleOptionTransformer(string $name): callable
    {}

    /**
     * Apply the given transformation to each of the given input's values.
     *
     * @param  callable(string): string  $transformer
     * @return list<string>
     */
    protected function applyTransformationToConsoleInputValue(null|bool|string|array $value, callable $transformer): array
    {}

    /**
     * Determine if the given command argument or option should be redacted.
     */
    protected function shouldRedactConsoleInputValue(string $name): bool
    {}

    /**
     * Retrieve the console input of the currently running Artisan command.
     */
    protected function currentConsoleInput(): ConsoleInput
    {}

    /**
     * Retrieve the fallback console input built from the process arguments.
     */
    protected function currentFallbackArgvInput(): ArgvInput
    {}

    /**
     * Prepare to run the given scheduled task.
     */
    public function prepareForScheduledTask(ScheduledTask $task): void
    {}

    /**
     * Handle the given scheduled task finishing.
     */
    public function finishScheduledTask(ScheduledTask $task): void
    {}

    /**
     * Flush the currently running scheduled task context.
     */
    public function flushScheduledTaskContext(): void
    {}

    /**
     * Determine if a scheduled task is running.
     */
    protected function isRunningScheduledTask(): bool
    {}

    /**
     * Retrieve the currently running scheduled task's execution context.
     *
     * @return array<string, mixed>
     */
    protected function scheduledTaskExecutionDetails(Throwable $e): array
    {}

    /**
     * Retrieve the name of the currently running scheduled task.
     */
    protected function scheduledTaskName(): string
    {}

    /**
     * Retrieve the name of the currently running scheduled callback task.
     */
    protected function scheduledCallbackTaskName(): string
    {}

    /**
     * Retrieve the name of the given scheduled closure task.
     */
    protected function scheduledClosureTaskName(Closure $callback): string
    {}

    /**
     * Retrieve the class name of the currently running scheduled task.
     */
    protected function scheduledTaskClass(): ?string
    {}

    /**
     * Retrieve the class name of the currently running scheduled callback task.
     */
    protected function scheduledCallbackTaskClass(): ?string
    {}

    /**
     * Retrieve the callback of the currently running scheduled task.
     */
    protected function scheduledTaskCallback(): mixed
    {}

    /**
     * Retrieve the timezone of the currently running scheduled task.
     */
    protected function scheduledTaskTimezone(): ?string
    {}

    /**
     * Prepare to process the given job.
     */
    public function prepareForJob(Job $job): void
    {}

    /**
     * Flush currently processing job context.
     */
    public function flushJobContext(): void
    {}

    /**
     * Determine if a queue worker is running.
     */
    protected function isProcessingJob(): bool
    {}

    /**
     * Retrieve the currently processing job's execution context.
     *
     * @return array<string, mixed>
     */
    protected function jobExecutionDetails(Throwable $e): array
    {}

    /**
     * Retrieve the trace identifier the current job belongs to.
     */
    protected function traceIdFromContext(): ?string
    {}

    /**
     * The currently processing job attempt ID.
     */
    protected function currentlyProcessingJobAttemptId(): string
    {}

    /**
     * Normalize the given queue name.
     */
    protected function normalizedQueue(): string
    {}

    /**
     * Remember the given user as they log out.
     */
    public function rememberUser(Authenticatable $user): void
    {}

    /**
     * Handle context dehydrating.
     */
    public function handleContextDehydrating(ContextRepository $context): void
    {}

    /**
     * Capture the authenticated user's identifier in the given context.
     */
    protected function rememberUserIdInContext(ContextRepository $context): void
    {}

    /**
     * Capture the trace identifier of the current execution in the given context.
     */
    protected function rememberTraceIdInContext(ContextRepository $context): void
    {}

    /**
     * Retrieve the current user ID.
     */
    protected function userId(): ?string
    {}

    /**
     * Retrieve the authenticated user's identifier from the context.
     */
    protected function userIdFromContext(): ?string
    {}

    /**
     * Retrieve the identifier of the given user.
     */
    protected function userIdentifier(Authenticatable $user): ?string
    {}

    /**
     * Determine if the exception was handled.
     */
    protected function handled(Throwable $e): bool
    {}

    /**
     * Parse the given exception.
     *
     * @return array<string, mixed>
     */
    protected function parseException(Throwable $e): array
    {}

    /**
     * Parse the previous exceptions.
     *
     * @return list<array<string, mixed>>
     */
    protected function previous(Throwable $e): array
    {}

    /**
     * Parse the stack trace for the given exception.
     *
     * @return list<array<string, mixed>>
     */
    protected function trace(Throwable $e): array
    {}

    /**
     * Determine the type information of the given argument.
     *
     * @return string|array{string, string}
     */
    protected function argType(mixed $argument, string|int $name): string|array
    {}

    /**
     * Normalize the file and line number.
     *
     * @param  array<string, mixed>  $frame
     * @return array<string, mixed>
     */
    protected function normalizeFrameFileAndLine(array $frame): array
    {}

    /**
     * Map compiled views to their original view file paths.
     *
     * @param  array<string, mixed>  $frame
     * @return array<string, mixed>
     */
    protected function mapCompiledViewFrame(array $frame): array
    {}

    /**
     * Retrieve the Blade mapper.
     *
     * The mapper is only needed while reporting a view exception, and is
     * resolved when it is first needed.
     */
    protected function bladeMapper(): BladeMapper
    {}

    /**
     * Redact the given value.
     */
    protected function redactValue(string $value): string
    {}

    /**
     * Normalize the file path's base path.
     */
    protected function normalizeBasePath(string $path): string
    {}

    /**
     * Retrieve the time Laravel started.
     */
    protected function laravelStartedAtTimestamp(): ?string
    {}
}
