<?php

namespace Illuminate\Notifications;

use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldBeEncrypted;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Contracts\Queue\ShouldQueueAfterCommit;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Queue\Attributes\Backoff;
use Illuminate\Queue\Attributes\CountCrashesAsExceptions;
use Illuminate\Queue\Attributes\DeleteWhenMissingModels;
use Illuminate\Queue\Attributes\FailOnTimeout;
use Illuminate\Queue\Attributes\MaxExceptions;
use Illuminate\Queue\Attributes\ReadsQueueAttributes;
use Illuminate\Queue\Attributes\Timeout;
use Illuminate\Queue\Attributes\Tries;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\SerializesModels;
use Illuminate\Support\Collection;

class SendQueuedNotifications implements ShouldQueue
{
    use InteractsWithQueue, Queueable, ReadsQueueAttributes, SerializesModels;

    /**
     * The notifiable entities that should receive the notification.
     *
     * @var \Illuminate\Support\Collection
     */
    public $notifiables;

    /**
     * The notification to be sent.
     *
     * @var \Illuminate\Notifications\Notification
     */
    public $notification;

    /**
     * All of the channels to send the notification to.
     *
     * @var array|null
     */
    public $channels;

    /**
     * The number of times the job may be attempted.
     *
     * @var int
     */
    public $tries;

    /**
     * The number of seconds the job can run before timing out.
     *
     * @var int
     */
    public $timeout;

    /**
     * The maximum number of unhandled exceptions to allow before failing.
     *
     * @var int
     */
    public $maxExceptions;

    /**
     * Indicates if the job should be encrypted.
     *
     * @var bool
     */
    public $shouldBeEncrypted = false;

    /**
     * Indicates if the job should be deleted when models are missing.
     */
    public bool $deleteWhenMissingModels = false;

    /**
     * Indicates if the job should be marked as failed on timeout.
     *
     * @var bool
     */
    public $failOnTimeout = false;

    /**
     * Indicates if worker crashes should count towards the maximum exceptions.
     *
     * @var bool
     */
    public $countCrashesAsExceptions = false;

    /**
     * Create a new job instance.
     *
     * @param  \Illuminate\Notifications\Notifiable|\Illuminate\Support\Collection  $notifiables
     * @param  \Illuminate\Notifications\Notification  $notification
     * @param  array|null  $channels
     */
    public function __construct($notifiables, $notification, ?array $channels = null)
    {}

    /**
     * Wrap the notifiable(s) in a collection.
     *
     * @param  \Illuminate\Notifications\Notifiable|\Illuminate\Support\Collection  $notifiables
     * @return \Illuminate\Support\Collection
     */
    protected function wrapNotifiables($notifiables)
    {}

    /**
     * Send the notifications.
     *
     * @param  \Illuminate\Notifications\ChannelManager  $manager
     * @return void
     */
    public function handle(ChannelManager $manager)
    {}

    /**
     * Get the display name for the queued job.
     *
     * @return string
     */
    public function displayName()
    {}

    /**
     * Call the failed method on the notification instance.
     *
     * @param  \Throwable  $e
     * @return void
     */
    public function failed($e)
    {}

    /**
     * Get the number of seconds before a released notification will be available.
     *
     * @return mixed
     */
    public function backoff()
    {}

    /**
     * Determine the time at which the job should timeout.
     *
     * @return \DateTime|null
     */
    public function retryUntil()
    {}

    /**
     * Prepare the instance for cloning.
     *
     * @return void
     */
    public function __clone()
    {}
}
