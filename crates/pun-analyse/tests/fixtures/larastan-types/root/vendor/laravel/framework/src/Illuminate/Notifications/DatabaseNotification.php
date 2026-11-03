<?php

namespace Illuminate\Notifications;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\HasCollection;
use Illuminate\Database\Eloquent\Model;

class DatabaseNotification extends Model
{
    /** @use HasCollection<DatabaseNotificationCollection> */
    use HasCollection;

    /**
     * The "type" of the primary key ID.
     *
     * @var string
     */
    protected $keyType = 'string';

    /**
     * Indicates if the IDs are auto-incrementing.
     *
     * @var bool
     */
    public $incrementing = false;

    /**
     * The table associated with the model.
     *
     * @var string
     */
    protected $table = 'notifications';

    /**
     * The guarded attributes on the model.
     *
     * @var array<string>
     */
    protected $guarded = [];

    /**
     * The attributes that should be cast to native types.
     *
     * @var array
     */
    protected $casts = [
        'data' => 'array',
        'read_at' => 'datetime',
    ];

    /**
     * The type of collection that should be used for the model.
     */
    protected static string $collectionClass = DatabaseNotificationCollection::class;

    /**
     * Get the notifiable entity that the notification belongs to.
     *
     * @return \Illuminate\Database\Eloquent\Relations\MorphTo<\Illuminate\Database\Eloquent\Model, $this>
     */
    public function notifiable()
    {}

    /**
     * Mark the notification as read.
     *
     * @return void
     */
    public function markAsRead()
    {}

    /**
     * Mark the notification as unread.
     *
     * @return void
     */
    public function markAsUnread()
    {}

    /**
     * Determine if a notification has been read.
     *
     * @return bool
     */
    public function read()
    {}

    /**
     * Determine if a notification has not been read.
     *
     * @return bool
     */
    public function unread()
    {}

    /**
     * Scope a query to only include read notifications.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public function scopeRead(Builder $query)
    {}

    /**
     * Scope a query to only include unread notifications.
     *
     * @param  \Illuminate\Database\Eloquent\Builder<static>  $query
     * @return \Illuminate\Database\Eloquent\Builder<static>
     */
    public function scopeUnread(Builder $query)
    {}
}
