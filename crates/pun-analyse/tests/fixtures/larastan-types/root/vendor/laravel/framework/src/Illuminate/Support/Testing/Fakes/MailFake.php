<?php

namespace Illuminate\Support\Testing\Fakes;

use BackedEnum;
use Closure;
use Illuminate\Contracts\Mail\Factory;
use Illuminate\Contracts\Mail\Mailable;
use Illuminate\Contracts\Mail\Mailer;
use Illuminate\Contracts\Mail\MailQueue;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Mail\MailManager;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\ForwardsCalls;
use Illuminate\Support\Traits\ReflectsClosures;
use PHPUnit\Framework\Assert as PHPUnit;

class MailFake implements Factory, Fake, Mailer, MailQueue
{
    use ForwardsCalls, ReflectsClosures;

    /**
     * The mailer instance.
     *
     * @var MailManager
     */
    public $manager;

    /**
     * The mailer currently being used to send a message.
     *
     * @var string|null
     */
    protected $currentMailer;

    /**
     * All of the mailables that have been sent.
     *
     * @var array
     */
    protected $mailables = [];

    /**
     * All of the mailables that have been queued.
     *
     * @var array
     */
    protected $queuedMailables = [];

    /**
     * Create a new mail fake.
     *
     * @param  MailManager  $manager
     */
    public function __construct(MailManager $manager)
    {}

    /**
     * Assert if a mailable was sent based on a truth-test callback.
     *
     * @param  string|\Closure  $mailable
     * @param  callable|array|string|int|null  $callback
     * @return void
     */
    public function assertSent($mailable, $callback = null)
    {}

    /**
     * Assert if a mailable was sent a number of times.
     *
     * @param  string  $mailable
     * @param  int  $times
     * @return void
     */
    public function assertSentTimes($mailable, $times = 1)
    {}

    /**
     * Assert if a mailable was sent exactly once.
     *
     * @param  string  $mailable
     * @return void
     */
    public function assertSentOnce($mailable)
    {}

    /**
     * Determine if a mailable was not sent or queued to be sent based on a truth-test callback.
     *
     * @param  string|\Closure  $mailable
     * @param  callable|null  $callback
     * @return void
     */
    public function assertNotOutgoing($mailable, $callback = null)
    {}

    /**
     * Determine if a mailable was not sent based on a truth-test callback.
     *
     * @param  string|\Closure  $mailable
     * @param  callable|array|string|null  $callback
     * @return void
     */
    public function assertNotSent($mailable, $callback = null)
    {}

    /**
     * Assert that no mailables were sent or queued to be sent.
     *
     * @return void
     */
    public function assertNothingOutgoing()
    {}

    /**
     * Assert that no mailables were sent.
     *
     * @return void
     */
    public function assertNothingSent()
    {}

    /**
     * Assert if a mailable was queued based on a truth-test callback.
     *
     * @param  string|\Closure  $mailable
     * @param  callable|array|string|int|null  $callback
     * @return void
     */
    public function assertQueued($mailable, $callback = null)
    {}

    /**
     * Assert if a mailable was queued a number of times.
     *
     * @param  string  $mailable
     * @param  int  $times
     * @return void
     */
    public function assertQueuedTimes($mailable, $times = 1)
    {}

    /**
     * Assert if a mailable was queued exactly once.
     *
     * @param  string  $mailable
     * @return void
     */
    public function assertQueuedOnce($mailable)
    {}

    /**
     * Determine if a mailable was not queued based on a truth-test callback.
     *
     * @param  string|\Closure  $mailable
     * @param  callable|array|string|null  $callback
     * @return void
     */
    public function assertNotQueued($mailable, $callback = null)
    {}

    /**
     * Assert that no mailables were queued.
     *
     * @return void
     */
    public function assertNothingQueued()
    {}

    /**
     * Assert the total number of mailables that were sent.
     *
     * @param  int  $count
     * @return void
     */
    public function assertSentCount($count)
    {}

    /**
     * Assert the total number of mailables that were queued.
     *
     * @param  int  $count
     * @return void
     */
    public function assertQueuedCount($count)
    {}

    /**
     * Assert the total number of mailables that were sent or queued.
     *
     * @param  int  $count
     * @return void
     */
    public function assertOutgoingCount($count)
    {}

    /**
     * Get all of the mailables matching a truth-test callback.
     *
     * @param  string|\Closure  $mailable
     * @param  callable|null  $callback
     * @return \Illuminate\Support\Collection
     */
    public function sent($mailable, $callback = null)
    {}

    /**
     * Determine if the given mailable has been sent.
     *
     * @param  string  $mailable
     * @return bool
     */
    public function hasSent($mailable)
    {}

    /**
     * Get all of the queued mailables matching a truth-test callback.
     *
     * @param  string|\Closure  $mailable
     * @param  callable|null  $callback
     * @return \Illuminate\Support\Collection
     */
    public function queued($mailable, $callback = null)
    {}

    /**
     * Determine if the given mailable has been queued.
     *
     * @param  string  $mailable
     * @return bool
     */
    public function hasQueued($mailable)
    {}

    /**
     * Get all of the mailed mailables for a given type.
     *
     * @param  string  $type
     * @return \Illuminate\Support\Collection
     */
    protected function mailablesOf($type)
    {}

    /**
     * Get all of the mailed mailables for a given type.
     *
     * @param  string  $type
     * @return \Illuminate\Support\Collection
     */
    protected function queuedMailablesOf($type)
    {}

    /**
     * Get a mailer instance by name.
     *
     * @param  string|null  $name
     * @return \Illuminate\Contracts\Mail\Mailer
     */
    public function mailer($name = null)
    {}

    /**
     * Get a mailer driver instance.
     *
     * @param  string|null  $driver
     * @return \Illuminate\Contracts\Mail\Mailer
     */
    public function driver($driver = null)
    {}

    /**
     * Begin the process of mailing a mailable class instance.
     *
     * @param  mixed  $users
     * @return \Illuminate\Mail\PendingMail
     */
    public function to($users)
    {}

    /**
     * Begin the process of mailing a mailable class instance.
     *
     * @param  mixed  $users
     * @return \Illuminate\Mail\PendingMail
     */
    public function cc($users)
    {}

    /**
     * Begin the process of mailing a mailable class instance.
     *
     * @param  mixed  $users
     * @return \Illuminate\Mail\PendingMail
     */
    public function bcc($users)
    {}

    /**
     * Send a new message with only an HTML part.
     *
     * @param  string  $html
     * @param  \Closure|string  $callback
     * @return void
     */
    public function html($html, $callback)
    {}

    /**
     * Send a new message with only a raw text part.
     *
     * @param  string  $text
     * @param  \Closure|string  $callback
     * @return void
     */
    public function raw($text, $callback)
    {}

    /**
     * Send a new message with only a plain part.
     *
     * @param  string  $view
     * @param  array  $data
     * @param  \Closure|string  $callback
     * @return void
     */
    public function plain($view, array $data, $callback)
    {}

    /**
     * Send a new message using a view.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable|string|array  $view
     * @param  array  $data
     * @param  \Closure|string|null  $callback
     * @return mixed|void
     */
    public function send($view, array $data = [], $callback = null)
    {}

    /**
     * Send a new message synchronously using a view.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable|string|array  $mailable
     * @param  array  $data
     * @param  \Closure|string|null  $callback
     * @return void
     */
    public function sendNow($mailable, array $data = [], $callback = null)
    {}

    /**
     * Send a new message using a view.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable|string|array  $view
     * @param  bool  $shouldQueue
     * @return mixed|void
     */
    protected function sendMail($view, $shouldQueue = false)
    {}

    /**
     * Queue a new message for sending.
     *
     * @param  \Illuminate\Contracts\Mail\Mailable|string|array  $view
     * @param  \BackedEnum|string|null  $queue
     * @return mixed
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
     * @param  \BackedEnum|string|null  $queue
     * @param  \Illuminate\Contracts\Mail\Mailable  $view
     * @return mixed
     */
    public function queueOn($queue, $view)
    {}

    /**
     * Queue a new e-mail message for sending after (n) seconds.
     *
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  \Illuminate\Contracts\Mail\Mailable|string|array  $view
     * @param  string|null  $queue
     * @return mixed
     */
    public function later($delay, $view, $queue = null)
    {}

    /**
     * Queue a new e-mail message for sending after (n) seconds on the given queue.
     *
     * @param  string|null  $queue
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  \Illuminate\Contracts\Mail\Mailable  $view
     * @return mixed
     */
    public function laterOn($queue, $delay, $view)
    {}

    /**
     * Infer mailable class using reflection if a typehinted closure is passed to assertion.
     *
     * @param  string|\Closure  $mailable
     * @param  callable|null  $callback
     * @return array
     */
    protected function prepareMailableAndCallback($mailable, $callback)
    {}

    /**
     * Forget all of the resolved mailer instances.
     *
     * @return $this
     */
    public function forgetMailers()
    {}

    /**
     * Handle dynamic method calls to the mailer.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
