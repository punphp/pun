<?php

namespace Illuminate\Console\Scheduling;

/**
 * @mixin \Illuminate\Console\Scheduling\Schedule
 */
class PendingEventAttributes
{
    use ManagesAttributes, ManagesFrequencies;

    /**
     * Event lifecycle and output methods that should be deferred and replayed on each event in the group.
     *
     * @var array<int, string>
     */
    public const DEFERRED_EVENT_METHODS = [
        'before',
        'after',
        'then',
        'thenWithOutput',
        'onSuccess',
        'onSuccessWithOutput',
        'onFailure',
        'onFailureWithOutput',
        'pingBefore',
        'pingBeforeIf',
        'thenPing',
        'thenPingIf',
        'pingOnSuccess',
        'pingOnSuccessIf',
        'pingOnFailure',
        'pingOnFailureIf',
        'sendOutputTo',
        'appendOutputTo',
        'emailOutputTo',
        'emailWrittenOutputTo',
        'emailOutputOnFailure',
    ];

    /**
     * The recorded macro and deferred method calls to replay on each event.
     *
     * @var array<int, array{string, array}>
     */
    protected array $macros = [];

    /**
     * Create a new pending event attributes instance.
     */
    public function __construct(
        protected Schedule $schedule,
    ) {}

    /**
     * Do not allow the event to overlap each other.
     *
     * The expiration time of the underlying cache lock may be specified in minutes.
     *
     * @param  int  $expiresAt
     * @param  bool  $releaseOnTerminationSignals
     * @return $this
     */
    public function withoutOverlapping($expiresAt = 1440, $releaseOnTerminationSignals = true)
    {}

    /**
     * Merge the current attributes into the given event.
     */
    public function mergeAttributes(Event $event): void
    {}

    /**
     * Proxy missing methods onto the underlying schedule.
     */
    public function __call(string $method, array $parameters): mixed
    {}
}
