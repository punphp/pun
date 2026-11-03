<?php

namespace Illuminate\Notifications\Channels;

use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Notifications\Events\BroadcastNotificationCreated;
use Illuminate\Notifications\Messages\BroadcastMessage;
use Illuminate\Notifications\Notification;
use RuntimeException;

class BroadcastChannel
{
    /**
     * The event dispatcher.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher
     */
    protected $events;

    /**
     * Create a new broadcast channel.
     *
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     */
    public function __construct(Dispatcher $events)
    {}

    /**
     * Send the given notification.
     *
     * @param  mixed  $notifiable
     * @param  \Illuminate\Notifications\Notification  $notification
     * @return array|null
     */
    public function send($notifiable, Notification $notification)
    {}

    /**
     * Get the data for the notification.
     *
     * @param  mixed  $notifiable
     * @param  \Illuminate\Notifications\Notification  $notification
     * @return mixed
     *
     * @throws \RuntimeException
     */
    protected function getData($notifiable, Notification $notification)
    {}
}
