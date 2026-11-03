<?php

namespace Illuminate\Mail;

use Illuminate\Contracts\Mail\Mailable as MailableContract;
use Illuminate\Contracts\Mail\Mailer as MailerContract;
use Illuminate\Contracts\Translation\HasLocalePreference;
use Illuminate\Support\Traits\Conditionable;

class PendingMail
{
    use Conditionable;

    /**
     * The mailer instance.
     *
     * @var \Illuminate\Contracts\Mail\Mailer
     */
    protected $mailer;

    /**
     * The locale of the message.
     *
     * @var string|null
     */
    protected $locale;

    /**
     * The "to" recipients of the message.
     *
     * @var array
     */
    protected $to = [];

    /**
     * The "cc" recipients of the message.
     *
     * @var array
     */
    protected $cc = [];

    /**
     * The "bcc" recipients of the message.
     *
     * @var array
     */
    protected $bcc = [];

    /**
     * Create a new mailable mailer instance.
     *
     * @param  \Illuminate\Contracts\Mail\Mailer  $mailer
     */
    public function __construct(MailerContract $mailer)
    {}

    /**
     * Set the locale of the message.
     *
     * @param  string|null  $locale
     * @return $this
     */
    public function locale($locale)
    {}

    /**
     * Set the recipients of the message.
     *
     * @param  mixed  $users
     * @return $this
     */
    public function to($users)
    {}

    /**
     * Set the recipients of the message.
     *
     * @param  mixed  $users
     * @return $this
     */
    public function cc($users)
    {}

    /**
     * Set the recipients of the message.
     *
     * @param  mixed  $users
     * @return $this
     */
    public function bcc($users)
    {}

    /**
     * Send a new mailable message instance.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable  $mailable
     * @return \Illuminate\Mail\SentMessage|null
     */
    public function send(MailableContract $mailable)
    {}

    /**
     * Send a new mailable message instance synchronously.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable  $mailable
     * @return \Illuminate\Mail\SentMessage|null
     */
    public function sendNow(MailableContract $mailable)
    {}

    /**
     * Push the given mailable onto the queue.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable  $mailable
     * @return mixed
     */
    public function queue(MailableContract $mailable)
    {}

    /**
     * Deliver the queued message after (n) seconds.
     *
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  \Illuminate\Contracts\Mail\Mailable  $mailable
     * @return mixed
     */
    public function later($delay, MailableContract $mailable)
    {}

    /**
     * Populate the mailable with the addresses.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable  $mailable
     * @return \Illuminate\Contracts\Mail\Mailable
     */
    protected function fill(MailableContract $mailable)
    {}
}
