<?php

namespace Illuminate\Notifications\Events;

use Illuminate\Broadcasting\PrivateChannel;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Broadcasting\ShouldBroadcast;
use Illuminate\Notifications\AnonymousNotifiable;
use Illuminate\Queue\SerializesModels;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;

class BroadcastNotificationCreated implements ShouldBroadcast
{
    use Queueable, SerializesModels;

    /**
     * Create a new event instance.
     *
     * @param  mixed  $notifiable  The notifiable entity who received the notification.
     * @param  \Illuminate\Notifications\Notification  $notification  The notification instance.
     * @param  array  $data  The notification data.
     */
    public function __construct(
        public $notifiable,
        public $notification,
        public $data = [],
    ) {}

    /**
     * Get the channels the event should broadcast on.
     *
     * @return array
     */
    public function broadcastOn()
    {}

    /**
     * Get the broadcast channel name for the event.
     *
     * @return array|string
     */
    protected function channelName()
    {}

    /**
     * Get the data that should be sent with the broadcasted event.
     *
     * @return array
     */
    public function broadcastWith()
    {}

    /**
     * Get the type of the notification being broadcast.
     *
     * @return string
     */
    public function broadcastType()
    {}

    /**
     * Get the event name of the notification being broadcast.
     *
     * @return string
     */
    public function broadcastAs()
    {}
}
