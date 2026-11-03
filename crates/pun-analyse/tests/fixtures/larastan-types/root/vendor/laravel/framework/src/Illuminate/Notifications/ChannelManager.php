<?php

namespace Illuminate\Notifications;

use Illuminate\Contracts\Bus\Dispatcher as Bus;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Contracts\Notifications\Dispatcher as DispatcherContract;
use Illuminate\Contracts\Notifications\Factory as FactoryContract;
use Illuminate\Support\Manager;
use Illuminate\Support\Queue\Concerns\ResolvesQueueRoutes;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;

class ChannelManager extends Manager implements DispatcherContract, FactoryContract
{
    use Macroable, ResolvesQueueRoutes;

    /**
     * The resolved notification sender instance.
     *
     * @var \Illuminate\Notifications\NotificationSender|null
     */
    protected $notificationSender;

    /**
     * The default channel used to deliver messages.
     *
     * @var string
     */
    protected $defaultChannel = 'mail';

    /**
     * The locale used when sending notifications.
     *
     * @var string|null
     */
    protected $locale;

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
     * Get a channel instance.
     *
     * @param  \UnitEnum|string|null  $name
     * @return mixed
     */
    public function channel($name = null)
    {}

    /**
     * Get a driver instance.
     *
     * @param  \UnitEnum|string|null  $driver
     * @return mixed
     */
    public function driver($driver = null)
    {}

    /**
     * Create an instance of the database driver.
     *
     * @return \Illuminate\Notifications\Channels\DatabaseChannel
     */
    protected function createDatabaseDriver()
    {}

    /**
     * Create an instance of the broadcast driver.
     *
     * @return \Illuminate\Notifications\Channels\BroadcastChannel
     */
    protected function createBroadcastDriver()
    {}

    /**
     * Create an instance of the mail driver.
     *
     * @return \Illuminate\Notifications\Channels\MailChannel
     */
    protected function createMailDriver()
    {}

    /**
     * Create a new driver instance.
     *
     * @param  string  $driver
     * @return mixed
     *
     * @throws \InvalidArgumentException
     */
    protected function createDriver($driver)
    {}

    /**
     * Resolve the NotificationSender instance.
     *
     * @return \Illuminate\Notifications\NotificationSender
     */
    protected function resolveNotificationSender()
    {}

    /**
     * Get the default channel driver name.
     *
     * @return string
     */
    public function getDefaultDriver()
    {}

    /**
     * Get the default channel driver name.
     *
     * @return string
     */
    public function deliversVia()
    {}

    /**
     * Set the default channel driver name.
     *
     * @param  string  $channel
     * @return void
     */
    public function deliverVia($channel)
    {}

    /**
     * Set the locale of notifications.
     *
     * @param  string  $locale
     * @return $this
     */
    public function locale($locale)
    {}
}
