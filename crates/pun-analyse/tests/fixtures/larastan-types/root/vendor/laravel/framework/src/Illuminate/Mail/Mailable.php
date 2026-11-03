<?php

namespace Illuminate\Mail;

use Illuminate\Config\Repository as ConfigRepository;
use Illuminate\Container\Container;
use Illuminate\Contracts\Filesystem\Factory as FilesystemFactory;
use Illuminate\Contracts\Mail\Attachable;
use Illuminate\Contracts\Mail\Factory as MailFactory;
use Illuminate\Contracts\Mail\Mailable as MailableContract;
use Illuminate\Contracts\Queue\Factory as Queue;
use Illuminate\Contracts\Support\Htmlable;
use Illuminate\Contracts\Support\Renderable;
use Illuminate\Contracts\Translation\HasLocalePreference;
use Illuminate\Queue\Attributes\Connection;
use Illuminate\Queue\Attributes\Delay;
use Illuminate\Queue\Attributes\Queue as QueueAttribute;
use Illuminate\Support\Collection;
use Illuminate\Support\EncodedHtmlString;
use Illuminate\Support\HtmlString;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\ForwardsCalls;
use Illuminate\Support\Traits\Localizable;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\ReadsClassAttributes;
use Illuminate\Support\Traits\Tappable;
use Illuminate\Testing\Constraints\SeeInOrder;
use PHPUnit\Framework\Assert as PHPUnit;
use ReflectionClass;
use ReflectionProperty;
use Symfony\Component\Mailer\Header\MetadataHeader;
use Symfony\Component\Mailer\Header\TagHeader;
use Symfony\Component\Mime\Address;

class Mailable implements MailableContract, Renderable
{
    use Conditionable, ForwardsCalls, Localizable, ReadsClassAttributes, Tappable, Macroable {
        __call as macroCall;
    }

    /**
     * The locale of the message.
     *
     * @var string|null
     */
    public $locale;

    /**
     * The person the message is from.
     *
     * @var array
     */
    public $from = [];

    /**
     * The "to" recipients of the message.
     *
     * @var array
     */
    public $to = [];

    /**
     * The "cc" recipients of the message.
     *
     * @var array
     */
    public $cc = [];

    /**
     * The "bcc" recipients of the message.
     *
     * @var array
     */
    public $bcc = [];

    /**
     * The "reply to" recipients of the message.
     *
     * @var array
     */
    public $replyTo = [];

    /**
     * The subject of the message.
     *
     * @var string
     */
    public $subject;

    /**
     * The Markdown template for the message (if applicable).
     *
     * @var string|null
     */
    public $markdown;

    /**
     * The HTML to use for the message.
     *
     * @var string|null
     */
    protected $html;

    /**
     * The view to use for the message.
     *
     * @var string|null
     */
    public $view;

    /**
     * The plain text view to use for the message.
     *
     * @var string|null
     */
    public $textView;

    /**
     * The view data for the message.
     *
     * @var array
     */
    public $viewData = [];

    /**
     * The attachments for the message.
     *
     * @var array
     */
    public $attachments = [];

    /**
     * The raw attachments for the message.
     *
     * @var array
     */
    public $rawAttachments = [];

    /**
     * The attachments from a storage disk.
     *
     * @var array
     */
    public $diskAttachments = [];

    /**
     * The tags for the message.
     *
     * @var array
     */
    protected $tags = [];

    /**
     * The metadata for the message.
     *
     * @var array
     */
    protected $metadata = [];

    /**
     * The callbacks for the message.
     *
     * @var array
     */
    public $callbacks = [];

    /**
     * The name of the theme that should be used when formatting the message.
     *
     * @var string|null
     */
    public $theme;

    /**
     * The name of the mailer that should send the message.
     *
     * @var string
     */
    public $mailer;

    /**
     * The rendered mailable views for testing / assertions.
     *
     * @var array
     */
    protected $assertionableRenderStrings;

    /**
     * The callback that should be invoked while building the view data.
     *
     * @var callable
     */
    public static $viewDataCallback;

    /**
     * Send the message using the given mailer.
     *
     * @param  \Illuminate\Contracts\Mail\Factory|\Illuminate\Contracts\Mail\Mailer  $mailer
     * @return \Illuminate\Mail\SentMessage|null
     */
    public function send($mailer)
    {}

    /**
     * Queue the message for sending.
     *
     * @param  \Illuminate\Contracts\Queue\Factory  $queue
     * @return mixed
     */
    public function queue(Queue $queue)
    {}

    /**
     * Deliver the queued message after (n) seconds.
     *
     * @param  \DateTimeInterface|\DateInterval|int  $delay
     * @param  \Illuminate\Contracts\Queue\Factory  $queue
     * @return mixed
     */
    public function later($delay, Queue $queue)
    {}

    /**
     * Make the queued mailable job instance.
     *
     * @return mixed
     */
    protected function newQueuedJob()
    {}

    /**
     * Render the mailable into a view.
     *
     * @return string
     *
     * @throws \ReflectionException
     */
    public function render()
    {}

    /**
     * Build the view for the message.
     *
     * @return array|string
     *
     * @throws \ReflectionException
     */
    protected function buildView()
    {}

    /**
     * Build the Markdown view for the message.
     *
     * @return array
     *
     * @throws \ReflectionException
     */
    protected function buildMarkdownView()
    {}

    /**
     * Build the view data for the message.
     *
     * @return array
     *
     * @throws \ReflectionException
     */
    public function buildViewData()
    {}

    /**
     * Get additional meta-data to pass along with the view data.
     *
     * @return array<string, mixed>
     */
    protected function additionalMessageData(): array
    {}

    /**
     * Build the HTML view for a Markdown message.
     *
     * @param  array  $viewData
     * @return \Closure
     */
    protected function buildMarkdownHtml($viewData)
    {}

    /**
     * Build the text view for a Markdown message.
     *
     * @param  array  $viewData
     * @return \Closure
     */
    protected function buildMarkdownText($viewData)
    {}

    /**
     * Resolves a Markdown instance with the mail's theme.
     *
     * @return \Illuminate\Mail\Markdown
     */
    protected function markdownRenderer()
    {}

    /**
     * Add the sender to the message.
     *
     * @param  \Illuminate\Mail\Message  $message
     * @return $this
     */
    protected function buildFrom($message)
    {}

    /**
     * Add all of the recipients to the message.
     *
     * @param  \Illuminate\Mail\Message  $message
     * @return $this
     */
    protected function buildRecipients($message)
    {}

    /**
     * Set the subject for the message.
     *
     * @param  \Illuminate\Mail\Message  $message
     * @return $this
     */
    protected function buildSubject($message)
    {}

    /**
     * Add all of the attachments to the message.
     *
     * @param  \Illuminate\Mail\Message  $message
     * @return $this
     */
    protected function buildAttachments($message)
    {}

    /**
     * Add all of the disk attachments to the message.
     *
     * @param  \Illuminate\Mail\Message  $message
     * @return void
     */
    protected function buildDiskAttachments($message)
    {}

    /**
     * Add all defined tags to the message.
     *
     * @param  \Illuminate\Mail\Message  $message
     * @return $this
     */
    protected function buildTags($message)
    {}

    /**
     * Add all defined metadata to the message.
     *
     * @param  \Illuminate\Mail\Message  $message
     * @return $this
     */
    protected function buildMetadata($message)
    {}

    /**
     * Run the callbacks for the message.
     *
     * @param  \Illuminate\Mail\Message  $message
     * @return $this
     */
    protected function runCallbacks($message)
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
     * Set the priority of this message.
     *
     * The value is an integer where 1 is the highest priority and 5 is the lowest.
     *
     * @param  int  $level
     * @return $this
     */
    public function priority($level = 3)
    {}

    /**
     * Set the sender of the message.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return $this
     */
    public function from($address, $name = null)
    {}

    /**
     * Determine if the given recipient is set on the mailable.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return bool
     */
    public function hasFrom($address, $name = null)
    {}

    /**
     * Set the recipients of the message.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return $this
     */
    public function to($address, $name = null)
    {}

    /**
     * Determine if the given recipient is set on the mailable.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return bool
     */
    public function hasTo($address, $name = null)
    {}

    /**
     * Set the recipients of the message.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return $this
     */
    public function cc($address, $name = null)
    {}

    /**
     * Determine if the given recipient is set on the mailable.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return bool
     */
    public function hasCc($address, $name = null)
    {}

    /**
     * Set the recipients of the message.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return $this
     */
    public function bcc($address, $name = null)
    {}

    /**
     * Determine if the given recipient is set on the mailable.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return bool
     */
    public function hasBcc($address, $name = null)
    {}

    /**
     * Set the "reply to" address of the message.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return $this
     */
    public function replyTo($address, $name = null)
    {}

    /**
     * Determine if the given replyTo is set on the mailable.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return bool
     */
    public function hasReplyTo($address, $name = null)
    {}

    /**
     * Set the recipients of the message.
     *
     * All recipients are stored internally as [['name' => ?, 'address' => ?]]
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @param  string  $property
     * @return $this
     */
    protected function setAddress($address, $name = null, $property = 'to')
    {}

    /**
     * Convert the given recipient arguments to an array.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return array|\Illuminate\Support\Collection
     */
    protected function addressesToArray($address, $name)
    {}

    /**
     * Convert the given recipient into an object.
     *
     * @param  mixed  $recipient
     * @return object
     */
    protected function normalizeRecipient($recipient)
    {}

    /**
     * Determine if the given recipient is set on the mailable.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @param  string  $property
     * @return bool
     */
    protected function hasRecipient($address, $name = null, $property = 'to')
    {}

    /**
     * Determine if the mailable "envelope" method defines a recipient.
     *
     * @param  string  $address
     * @param  string|null  $name
     * @param  string  $property
     * @return bool
     */
    private function hasEnvelopeRecipient($address, $name, $property)
    {}

    /**
     * Set the subject of the message.
     *
     * @param  string  $subject
     * @return $this
     */
    public function subject($subject)
    {}

    /**
     * Determine if the mailable has the given subject.
     *
     * @param  string  $subject
     * @return bool
     */
    public function hasSubject($subject)
    {}

    /**
     * Set the Markdown template for the message.
     *
     * @param  string  $view
     * @param  array  $data
     * @return $this
     */
    public function markdown($view, array $data = [])
    {}

    /**
     * Set the view and view data for the message.
     *
     * @param  string  $view
     * @param  array  $data
     * @return $this
     */
    public function view($view, array $data = [])
    {}

    /**
     * Set the rendered HTML content for the message.
     *
     * @param  string  $html
     * @return $this
     */
    public function html($html)
    {}

    /**
     * Set the plain text view for the message.
     *
     * @param  string  $textView
     * @param  array  $data
     * @return $this
     */
    public function text($textView, array $data = [])
    {}

    /**
     * Set the view data for the message.
     *
     * @param  string|array  $key
     * @param  mixed  $value
     * @return $this
     */
    public function with($key, $value = null)
    {}

    /**
     * Attach a file to the message.
     *
     * @param  string|\Illuminate\Contracts\Mail\Attachable|\Illuminate\Mail\Attachment  $file
     * @param  array  $options
     * @return $this
     */
    public function attach($file, array $options = [])
    {}

    /**
     * Attach multiple files to the message.
     *
     * @param  array  $files
     * @return $this
     */
    public function attachMany($files)
    {}

    /**
     * Determine if the mailable has the given attachment.
     *
     * @param  string|\Illuminate\Contracts\Mail\Attachable|\Illuminate\Mail\Attachment  $file
     * @param  array  $options
     * @return bool
     */
    public function hasAttachment($file, array $options = [])
    {}

    /**
     * Determine if the mailable has the given envelope attachment.
     *
     * @param  \Illuminate\Mail\Attachment  $attachment
     * @param  array  $options
     * @return bool
     */
    private function hasEnvelopeAttachment($attachment, $options = [])
    {}

    /**
     * Attach a file to the message from storage.
     *
     * @param  string  $path
     * @param  string|null  $name
     * @param  array  $options
     * @return $this
     */
    public function attachFromStorage($path, $name = null, array $options = [])
    {}

    /**
     * Attach a file to the message from storage.
     *
     * @param  string|null  $disk
     * @param  string  $path
     * @param  string|null  $name
     * @param  array  $options
     * @return $this
     */
    public function attachFromStorageDisk($disk, $path, $name = null, array $options = [])
    {}

    /**
     * Determine if the mailable has the given attachment from storage.
     *
     * @param  string  $path
     * @param  string|null  $name
     * @param  array  $options
     * @return bool
     */
    public function hasAttachmentFromStorage($path, $name = null, array $options = [])
    {}

    /**
     * Determine if the mailable has the given attachment from a specific storage disk.
     *
     * @param  string|null  $disk
     * @param  string  $path
     * @param  string|null  $name
     * @param  array  $options
     * @return bool
     */
    public function hasAttachmentFromStorageDisk($disk, $path, $name = null, array $options = [])
    {}

    /**
     * Attach in-memory data as an attachment.
     *
     * @param  string  $data
     * @param  string  $name
     * @param  array  $options
     * @return $this
     */
    public function attachData($data, $name, array $options = [])
    {}

    /**
     * Determine if the mailable has the given data as an attachment.
     *
     * @param  string  $data
     * @param  string  $name
     * @param  array  $options
     * @return bool
     */
    public function hasAttachedData($data, $name, array $options = [])
    {}

    /**
     * Add a tag header to the message when supported by the underlying transport.
     *
     * @param  string  $value
     * @return $this
     */
    public function tag($value)
    {}

    /**
     * Determine if the mailable has the given tag.
     *
     * @param  string  $value
     * @return bool
     */
    public function hasTag($value)
    {}

    /**
     * Add a metadata header to the message when supported by the underlying transport.
     *
     * @param  array|string  $key
     * @param  string|null  $value
     * @return $this
     */
    public function metadata($key, $value = null)
    {}

    /**
     * Determine if the mailable has the given metadata.
     *
     * @param  string  $key
     * @param  string  $value
     * @return bool
     */
    public function hasMetadata($key, $value)
    {}

    /**
     * Assert that the mailable is from the given address.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return $this
     */
    public function assertFrom($address, $name = null)
    {}

    /**
     * Assert that the mailable has the given recipient.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return $this
     */
    public function assertTo($address, $name = null)
    {}

    /**
     * Assert that the mailable has the given recipient.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return $this
     */
    public function assertHasTo($address, $name = null)
    {}

    /**
     * Assert that the mailable has the given recipient.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return $this
     */
    public function assertHasCc($address, $name = null)
    {}

    /**
     * Assert that the mailable has the given recipient.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return $this
     */
    public function assertHasBcc($address, $name = null)
    {}

    /**
     * Assert that the mailable has the given "reply to" address.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return $this
     */
    public function assertHasReplyTo($address, $name = null)
    {}

    /**
     * Format the mailable recipient for display in an assertion message.
     *
     * @param  object|array|string  $address
     * @param  string|null  $name
     * @return string
     */
    private function formatAssertionRecipient($address, $name = null)
    {}

    /**
     * Format actual recipients for display in assertion messages.
     *
     * @param  array  $recipients
     * @return string
     */
    private function formatActualRecipients($recipients)
    {}

    /**
     * Assert that the mailable has the given subject.
     *
     * @param  string  $subject
     * @return $this
     */
    public function assertHasSubject($subject)
    {}

    /**
     * Assert that the given text is present in the HTML email body.
     *
     * @param  string  $string
     * @param  bool  $escape
     * @return $this
     */
    public function assertSeeInHtml($string, $escape = true)
    {}

    /**
     * Assert that the given text is not present in the HTML email body.
     *
     * @param  string  $string
     * @param  bool  $escape
     * @return $this
     */
    public function assertDontSeeInHtml($string, $escape = true)
    {}

    /**
     * Assert that the given text strings are present in order in the HTML email body.
     *
     * @param  array  $strings
     * @param  bool  $escape
     * @return $this
     */
    public function assertSeeInOrderInHtml($strings, $escape = true)
    {}

    /**
     * Assert that the given text is present in the plain-text email body.
     *
     * @param  string  $string
     * @return $this
     */
    public function assertSeeInText($string)
    {}

    /**
     * Assert that the given text is not present in the plain-text email body.
     *
     * @param  string  $string
     * @return $this
     */
    public function assertDontSeeInText($string)
    {}

    /**
     * Assert that the given text strings are present in order in the plain-text email body.
     *
     * @param  array  $strings
     * @return $this
     */
    public function assertSeeInOrderInText($strings)
    {}

    /**
     * Assert the mailable has the given attachment.
     *
     * @return $this
     */
    public function assertHasNoAttachments()
    {}

    public function assertHasAttachment($file, array $options = [])
    {}

    /**
     * Assert the mailable has the given data as an attachment.
     *
     * @param  string  $data
     * @param  string  $name
     * @param  array  $options
     * @return $this
     */
    public function assertHasAttachedData($data, $name, array $options = [])
    {}

    /**
     * Assert the mailable has the given attachment from storage.
     *
     * @param  string  $path
     * @param  string|null  $name
     * @param  array  $options
     * @return $this
     */
    public function assertHasAttachmentFromStorage($path, $name = null, array $options = [])
    {}

    /**
     * Assert the mailable has the given attachment from a specific storage disk.
     *
     * @param  string  $disk
     * @param  string  $path
     * @param  string|null  $name
     * @param  array  $options
     * @return $this
     */
    public function assertHasAttachmentFromStorageDisk($disk, $path, $name = null, array $options = [])
    {}

    /**
     * Assert that the mailable has the given tag.
     *
     * @param  string  $tag
     * @return $this
     */
    public function assertHasTag($tag)
    {}

    /**
     * Assert that the mailable has the given metadata.
     *
     * @param  string  $key
     * @param  string  $value
     * @return $this
     */
    public function assertHasMetadata($key, $value)
    {}

    /**
     * Render the HTML and plain-text version of the mailable into views for assertions.
     *
     * @return array
     *
     * @throws \ReflectionException
     */
    protected function renderForAssertions()
    {}

    /**
     * Prepare the mailable instance for delivery.
     *
     * @return void
     */
    protected function prepareMailableForDelivery()
    {}

    /**
     * Ensure the mailable's headers are hydrated from the "headers" method.
     *
     * @return void
     */
    private function ensureHeadersAreHydrated()
    {}

    /**
     * Ensure the mailable's "envelope" data is hydrated from the "envelope" method.
     *
     * @return void
     */
    private function ensureEnvelopeIsHydrated()
    {}

    /**
     * Ensure the mailable's content is hydrated from the "content" method.
     *
     * @return void
     */
    private function ensureContentIsHydrated()
    {}

    /**
     * Ensure the mailable's attachments are hydrated from the "attachments" method.
     *
     * @return void
     */
    private function ensureAttachmentsAreHydrated()
    {}

    /**
     * Determine if the mailable will be sent by the given mailer.
     *
     * @param  string  $mailer
     * @return bool
     */
    public function usesMailer($mailer)
    {}

    /**
     * Set the name of the mailer that should send the message.
     *
     * @param  string  $mailer
     * @return $this
     */
    public function mailer($mailer)
    {}

    /**
     * Register a callback to be called with the Symfony message instance.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function withSymfonyMessage($callback)
    {}

    /**
     * Register a callback to be called while building the view data.
     *
     * @param  callable  $callback
     * @return void
     */
    public static function buildViewDataUsing(callable $callback)
    {}

    /**
     * Dynamically bind parameters to the message.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return $this
     *
     * @throws \BadMethodCallException
     */
    public function __call($method, $parameters)
    {}
}
