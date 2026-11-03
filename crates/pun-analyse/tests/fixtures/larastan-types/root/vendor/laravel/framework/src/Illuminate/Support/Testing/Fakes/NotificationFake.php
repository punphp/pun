<?php

namespace Illuminate\Support\Testing\Fakes;

use Closure;
use Exception;
use Illuminate\Contracts\Notifications\Dispatcher as NotificationDispatcher;
use Illuminate\Contracts\Notifications\Factory as NotificationFactory;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Contracts\Translation\HasLocalePreference;
use Illuminate\Notifications\AnonymousNotifiable;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\ReflectsClosures;
use PHPUnit\Framework\Assert as PHPUnit;

class NotificationFake implements Fake, NotificationDispatcher, NotificationFactory
{
    use Macroable, MatchesProperties, ReflectsClosures;

    /**
     * All of the notifications that have been sent.
     *
     * @var array
     */
    protected $notifications = [];

    /**
     * Locale used when sending notifications.
     *
     * @var string|null
     */
    public $locale;

    /**
     * Indicates if notifications should be serialized and restored when pushed to the queue.
     *
     * @var bool
     */
    protected $serializeAndRestore = false;

    /**
     * Assert if a notification was sent on-demand based on a truth-test callback.
     *
     * @param  string|\Closure  $notification
     * @param  callable|null  $callback
     * @return void
     *
     * @throws \Exception
     */
    public function assertSentOnDemand($notification, $callback = null)
    {}

    /**
     * Assert if a notification was sent based on a truth-test callback.
     *
     * @param  mixed  $notifiable
     * @param  string|\Closure  $notification
     * @param  callable|array<string, mixed>|null  $callback
     * @return void
     *
     * @throws \Exception
     */
    public function assertSentTo($notifiable, $notification, $callback = null)
    {}

    /**
     * Assert if a notification was sent on-demand a number of times.
     *
     * @param  string  $notification
     * @param  int  $times
     * @return void
     */
    public function assertSentOnDemandTimes($notification, $times = 1)
    {}

    /**
     * Assert if a notification was sent on-demand exactly once.
     *
     * @param  string  $notification
     * @return void
     */
    public function assertSentOnDemandOnce($notification)
    {}

    /**
     * Assert if a notification was sent a number of times.
     *
     * @param  mixed  $notifiable
     * @param  string  $notification
     * @param  int  $times
     * @return void
     */
    public function assertSentToTimes($notifiable, $notification, $times = 1)
    {}

    /**
     * Assert if a notification was sent exactly once.
     *
     * @param  mixed  $notifiable
     * @param  string  $notification
     * @return void
     */
    public function assertSentToOnce($notifiable, $notification)
    {}

    /**
     * Determine if a notification was sent based on a truth-test callback.
     *
     * @param  mixed  $notifiable
     * @param  string|\Closure  $notification
     * @param  callable|array<string, mixed>|null  $callback
     * @return void
     *
     * @throws \Exception
     */
    public function assertNotSentTo($notifiable, $notification, $callback = null)
    {}

    /**
     * Assert that no notifications were sent.
     *
     * @return void
     */
    public function assertNothingSent()
    {}

    /**
     * Assert that no notifications were sent to the given notifiable.
     *
     * @param  mixed  $notifiable
     * @return void
     *
     * @throws \Exception
     */
    public function assertNothingSentTo($notifiable)
    {}

    /**
     * Assert the total amount of times a notification was sent.
     *
     * @param  string  $notification
     * @param  int  $expectedCount
     * @return void
     */
    public function assertSentTimes($notification, $expectedCount)
    {}

    /**
     * Assert the total count of notification that were sent.
     *
     * @param  int  $expectedCount
     * @return void
     */
    public function assertCount($expectedCount)
    {}

    /**
     * Get all of the notifications matching a truth-test callback.
     *
     * @param  mixed  $notifiable
     * @param  string  $notification
     * @param  callable|array<string, mixed>|null  $callback
     * @return \Illuminate\Support\Collection
     */
    public function sent($notifiable, $notification, $callback = null)
    {}

    /**
     * Determine if there are more notifications left to inspect.
     *
     * @param  mixed  $notifiable
     * @param  string  $notification
     * @return bool
     */
    public function hasSent($notifiable, $notification)
    {}

    /**
     * Get all of the notifications for a notifiable entity by type.
     *
     * @param  mixed  $notifiable
     * @param  string  $notification
     * @return array
     */
    protected function notificationsFor($notifiable, $notification)
    {}

    /**
     * Send the given notification to the given notifiable entities.
     *
     * @param  \Illuminate\Support\Collection|mixed  $notifiables
     * @param  mixed  $notification
     * @return void
     */
    public function send($notifiables, $notification)
    {}

    /**
     * Send the given notification immediately.
     *
     * @param  \Illuminate\Support\Collection|mixed  $notifiables
     * @param  mixed  $notification
     * @param  array|null  $channels
     * @return void
     */
    public function sendNow($notifiables, $notification, ?array $channels = null)
    {}

    /**
     * Get a channel instance by name.
     *
     * @param  string|null  $name
     * @return mixed
     */
    public function channel($name = null)
    {}

    /**
     * Set the locale of notifications.
     *
     * @param  string  $locale
     * @return $this
     */
    public function locale($locale)
    {}

    /**
     * Specify if notification should be serialized and restored when being "pushed" to the queue.
     *
     * @param  bool  $serializeAndRestore
     * @return $this
     */
    public function serializeAndRestore(bool $serializeAndRestore = true)
    {}

    /**
     * Serialize and unserialize the notification to simulate the queueing process.
     *
     * @param  mixed  $notification
     * @return mixed
     */
    protected function serializeAndRestoreNotification($notification)
    {}

    /**
     * Get the notifications that have been sent.
     *
     * @return array
     */
    public function sentNotifications()
    {}
}
