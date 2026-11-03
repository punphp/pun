<?php

namespace Illuminate\Mail;

use BackedEnum;
use Closure;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Contracts\Mail\Mailable as MailableContract;
use Illuminate\Contracts\Mail\Mailer as MailerContract;
use Illuminate\Contracts\Mail\MailQueue as MailQueueContract;
use Illuminate\Contracts\Queue\Factory as QueueContract;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Contracts\Support\Htmlable;
use Illuminate\Contracts\View\Factory;
use Illuminate\Mail\Events\MessageSending;
use Illuminate\Mail\Events\MessageSent;
use Illuminate\Mail\Mailables\Address;
use Illuminate\Support\HtmlString;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use Symfony\Component\Mailer\Envelope;
use Symfony\Component\Mailer\Transport\TransportInterface;
use Symfony\Component\Mime\Email;

class Mailer implements MailerContract, MailQueueContract
{
    use Macroable;

    /**
     * The name that is configured for the mailer.
     *
     * @var string
     */
    protected $name;

    /**
     * The view factory instance.
     *
     * @var \Illuminate\Contracts\View\Factory
     */
    protected $views;

    /**
     * The Symfony Transport instance.
     *
     * @var \Symfony\Component\Mailer\Transport\TransportInterface
     */
    protected $transport;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher|null
     */
    protected $events;

    /**
     * The global from address and name.
     *
     * @var array
     */
    protected $from;

    /**
     * The global reply-to address and name.
     *
     * @var array
     */
    protected $replyTo;

    /**
     * The global return path address.
     *
     * @var array
     */
    protected $returnPath;

    /**
     * The global to address and name.
     *
     * @var array
     */
    protected $to;

    /**
     * The queue factory implementation.
     *
     * @var \Illuminate\Contracts\Queue\Factory
     */
    protected $queue;

    /**
     * Create a new Mailer instance.
     *
     * @param  string  $name
     * @param  \Illuminate\Contracts\View\Factory  $views
     * @param  \Symfony\Component\Mailer\Transport\TransportInterface  $transport
     * @param  \Illuminate\Contracts\Events\Dispatcher|null  $events
     */
    public function __construct(string $name, Factory $views, TransportInterface $transport, ?Dispatcher $events = null)
    {}

    /**
     * Set the global from address and name.
     *
     * @param  string  $address
     * @param  string|null  $name
     * @return void
     */
    public function alwaysFrom($address, $name = null)
    {}

    /**
     * Set the global reply-to address and name.
     *
     * @param  string  $address
     * @param  string|null  $name
     * @return void
     */
    public function alwaysReplyTo($address, $name = null)
    {}

    /**
     * Set the global return path address.
     *
     * @param  string  $address
     * @return void
     */
    public function alwaysReturnPath($address)
    {}

    /**
     * Set the global to address and name.
     *
     * @param  string  $address
     * @param  string|null  $name
     * @return void
     */
    public function alwaysTo($address, $name = null)
    {}

    /**
     * Begin the process of mailing a mailable class instance.
     *
     * @param  mixed  $users
     * @param  string|null  $name
     * @return \Illuminate\Mail\PendingMail
     */
    public function to($users, $name = null)
    {}

    /**
     * Begin the process of mailing a mailable class instance.
     *
     * @param  mixed  $users
     * @param  string|null  $name
     * @return \Illuminate\Mail\PendingMail
     */
    public function cc($users, $name = null)
    {}

    /**
     * Begin the process of mailing a mailable class instance.
     *
     * @param  mixed  $users
     * @param  string|null  $name
     * @return \Illuminate\Mail\PendingMail
     */
    public function bcc($users, $name = null)
    {}

    /**
     * Send a new message with only an HTML part.
     *
     * @param  string  $html
     * @param  mixed  $callback
     * @return \Illuminate\Mail\SentMessage|null
     */
    public function html($html, $callback)
    {}

    /**
     * Send a new message with only a raw text part.
     *
     * @param  string  $text
     * @param  mixed  $callback
     * @return \Illuminate\Mail\SentMessage|null
     */
    public function raw($text, $callback)
    {}

    /**
     * Send a new message with only a plain part.
     *
     * @param  string  $view
     * @param  array  $data
     * @param  mixed  $callback
     * @return \Illuminate\Mail\SentMessage|null
     */
    public function plain($view, array $data, $callback)
    {}

    /**
     * Render the given message as a view.
     *
     * @param  string|array  $view
     * @param  array  $data
     * @return string
     */
    public function render($view, array $data = [])
    {}

    /**
     * Replace the embedded image attachments with raw, inline image data for browser rendering.
     *
     * @param  string  $renderedView
     * @param  array  $attachments
     * @return string
     */
    protected function replaceEmbeddedAttachments(string $renderedView, array $attachments)
    {}

    /**
     * Send a new message using a view.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable|string|array  $view
     * @param  array  $data
     * @param  \Closure|string|null  $callback
     * @return \Illuminate\Mail\SentMessage|null
     */
    public function send($view, array $data = [], $callback = null)
    {}

    /**
     * Send the given mailable.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable  $mailable
     * @return \Illuminate\Mail\SentMessage|null
     */
    protected function sendMailable(MailableContract $mailable)
    {}

    /**
     * Send a new message synchronously using a view.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable|string|array  $mailable
     * @param  array  $data
     * @param  \Closure|string|null  $callback
     * @return \Illuminate\Mail\SentMessage|null
     */
    public function sendNow($mailable, array $data = [], $callback = null)
    {}

    /**
     * Parse the given view name or array.
     *
     * @param  \Closure|array|string  $view
     * @return array
     *
     * @throws \InvalidArgumentException
     */
    protected function parseView($view)
    {}

    /**
     * Add the content to a given message.
     *
     * @param  \Illuminate\Mail\Message  $message
     * @param  string|null  $view
     * @param  string|null  $plain
     * @param  string|null  $raw
     * @param  array  $data
     * @return void
     */
    protected function addContent($message, $view, $plain, $raw, $data)
    {}

    /**
     * Render the given view.
     *
     * @param  \Closure|string  $view
     * @param  array  $data
     * @return string
     */
    protected function renderView($view, $data)
    {}

    /**
     * Set the global "to" address on the given message.
     *
     * @param  \Illuminate\Mail\Message  $message
     * @return void
     */
    protected function setGlobalToAndRemoveCcAndBcc($message)
    {}

    /**
     * Queue a new mail message for sending.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable  $view
     * @param  \BackedEnum|string|null  $queue
     * @return mixed
     *
     * @throws \InvalidArgumentException
     */
    public function queue($view, $queue = null)
    {}

    /**
     * Queue a new mail message for sending on the given queue.
     *
     * @param  \BackedEnum|string|null  $queue
     * @param  \Illuminate\Contracts\Mail\Mailable  $view
     * @return mixed
     */
    public function onQueue($queue, $view)
    {}

    /**
     * Queue a new mail message for sending on the given queue.
     *
     * This method didn't match rest of framework's "onQueue" phrasing. Added "onQueue".
     *
     * @param  string  $queue
     * @param  \Illuminate\Contracts\Mail\Mailable  $view
     * @return mixed
     */
    public function queueOn($queue, $view)
    {}

    /**
     * Queue a new mail message for sending after (n) seconds.
     *
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  \Illuminate\Contracts\Mail\Mailable  $view
     * @param  string|null  $queue
     * @return mixed
     *
     * @throws \InvalidArgumentException
     */
    public function later($delay, $view, $queue = null)
    {}

    /**
     * Queue a new mail message for sending after (n) seconds on the given queue.
     *
     * @param  string  $queue
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  \Illuminate\Contracts\Mail\Mailable  $view
     * @return mixed
     */
    public function laterOn($queue, $delay, $view)
    {}

    /**
     * Create a new message instance.
     *
     * @return \Illuminate\Mail\Message
     */
    protected function createMessage()
    {}

    /**
     * Send a Symfony Email instance.
     *
     * @param  \Symfony\Component\Mime\Email  $message
     * @return \Symfony\Component\Mailer\SentMessage|null
     */
    protected function sendSymfonyMessage(Email $message)
    {}

    /**
     * Determines if the email can be sent.
     *
     * @param  \Symfony\Component\Mime\Email  $message
     * @param  array  $data
     * @return bool
     */
    protected function shouldSendMessage($message, $data = [])
    {}

    /**
     * Dispatch the message sent event.
     *
     * @param  \Illuminate\Mail\SentMessage  $message
     * @param  array  $data
     * @return void
     */
    protected function dispatchSentEvent($message, $data = [])
    {}

    /**
     * Get the Symfony Transport instance.
     *
     * @return \Symfony\Component\Mailer\Transport\TransportInterface
     */
    public function getSymfonyTransport()
    {}

    /**
     * Get the view factory instance.
     *
     * @return \Illuminate\Contracts\View\Factory
     */
    public function getViewFactory()
    {}

    /**
     * Set the Symfony Transport instance.
     *
     * @param  \Symfony\Component\Mailer\Transport\TransportInterface  $transport
     * @return void
     */
    public function setSymfonyTransport(TransportInterface $transport)
    {}

    /**
     * Set the queue manager instance.
     *
     * @param  \Illuminate\Contracts\Queue\Factory  $queue
     * @return $this
     */
    public function setQueue(QueueContract $queue)
    {}
}
