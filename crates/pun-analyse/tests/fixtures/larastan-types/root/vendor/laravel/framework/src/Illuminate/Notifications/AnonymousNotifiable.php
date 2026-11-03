<?php

namespace Illuminate\Notifications;

use Illuminate\Contracts\Notifications\Dispatcher;
use InvalidArgumentException;

class AnonymousNotifiable
{
    /**
     * All of the notification routing information.
     *
     * @var array
     */
    public $routes = [];

    /**
     * Add routing information to the target.
     *
     * @param  string  $channel
     * @param  mixed  $route
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function route($channel, $route)
    {}

    /**
     * Send the given notification.
     *
     * @param  mixed  $notification
     * @return void
     */
    public function notify($notification)
    {}

    /**
     * Send the given notification immediately.
     *
     * @param  mixed  $notification
     * @return void
     */
    public function notifyNow($notification)
    {}

    /**
     * Get the notification routing information for the given driver.
     *
     * @param  string  $driver
     * @return mixed
     */
    public function routeNotificationFor($driver)
    {}

    /**
     * Get the value of the notifiable's primary key.
     *
     * @return mixed
     */
    public function getKey()
    {}
}
