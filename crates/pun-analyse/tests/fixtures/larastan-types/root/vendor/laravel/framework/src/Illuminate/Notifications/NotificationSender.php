<?php

namespace Illuminate\Notifications;

use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Contracts\Translation\HasLocalePreference;
use Illuminate\Database\Eloquent\Collection as EloquentCollection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Notifications\Events\NotificationFailed;
use Illuminate\Notifications\Events\NotificationSending;
use Illuminate\Notifications\Events\NotificationSent;
use Illuminate\Notifications\Events\NotificationSkipped;
use Illuminate\Queue\Attributes\Connection;
use Illuminate\Queue\Attributes\Delay;
use Illuminate\Queue\Attributes\Queue as QueueAttribute;
use Illuminate\Queue\Attributes\ReadsQueueAttributes;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Localizable;
use Symfony\Component\Mailer\Exception\HttpTransportException;
use Symfony\Component\Mailer\Exception\TransportException;
use Throwable;

class NotificationSender
{
    use Localizable, ReadsQueueAttributes;

    /**
     * The notification manager instance.
     *
     * @var \Illuminate\Notifications\ChannelManager
     */
    protected $manager;

    /**
     * The Bus dispatcher instance.
     *
     * @var \Illuminate\Contracts\Bus\Dispatcher
     */
    protected $bus;

    /**
     * The event dispatcher.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher
     */
    protected $events;

    /**
     * The locale to be used when sending notifications.
     *
     * @var string|null
     */
    protected $locale;

    /**
     * Indicates whether a NotificationFailed event has been dispatched.
     *
     * @var bool
     */
    protected $failedEventWasDispatched = false;

    /**
     * Create a new notification sender instance.
     *
     * @param  \Illuminate\Notifications\ChannelManager  $manager
     * @param  \Illuminate\Contracts\Bus\Dispatcher  $bus
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     * @param  string|null  $locale
     */
    public function __construct($manager, $bus, $events, $locale = null)
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
     * Get the notifiable's preferred locale for the notification.
     *
     * @param  mixed  $notifiable
     * @param  mixed  $notification
     * @return string|null
     */
    protected function preferredLocale($notifiable, $notification)
    {}

    /**
     * Send the given notification to the given notifiable via a channel.
     *
     * @param  mixed  $notifiable
     * @param  string  $id
     * @param  mixed  $notification
     * @param  string  $channel
     * @return void
     *
     * @throws \Throwable
     */
    protected function sendToNotifiable($notifiable, $id, $notification, $channel)
    {}

    /**
     * Determines if the notification can be sent.
     *
     * @param  mixed  $notifiable
     * @param  mixed  $notification
     * @param  string  $channel
     * @return bool
     */
    protected function shouldSendNotification($notifiable, $notification, $channel)
    {}

    /**
     * Queue the given notification instances.
     *
     * @param  mixed  $notifiables
     * @param  \Illuminate\Notifications\Notification  $notification
     * @return void
     */
    protected function queueNotification($notifiables, $notification)
    {}

    /**
     * Format the notifiables into a Collection / array if necessary.
     *
     * @param  mixed  $notifiables
     * @return \Illuminate\Database\Eloquent\Collection|array
     */
    protected function formatNotifiables($notifiables)
    {}

    /**
     * Set the locale to be used when sending notifications.
     *
     * @param  string|null  $locale
     * @return $this
     */
    public function locale($locale)
    {}
}
