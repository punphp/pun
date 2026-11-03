<?php

namespace Illuminate\Mail;

use Closure;
use Illuminate\Container\Container;
use Illuminate\Contracts\Filesystem\Factory as FilesystemFactory;
use Illuminate\Http\UploadedFile;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use RuntimeException;

class Attachment
{
    use Macroable;

    /**
     * The attached file's filename.
     *
     * @var string|null
     */
    public $as;

    /**
     * The attached file's MIME type.
     *
     * @var string|null
     */
    public $mime;

    /**
     * A callback that attaches the attachment to the mail message.
     *
     * @var \Closure
     */
    protected $resolver;

    /**
     * Create a mail attachment.
     *
     * @param  \Closure  $resolver
     */
    private function __construct(Closure $resolver)
    {}

    /**
     * Create a mail attachment from a path.
     *
     * @param  string  $path
     * @return static
     */
    public static function fromPath($path)
    {}

    /**
     * Create a mail attachment from a URL.
     *
     * @param  string  $url
     * @return static
     */
    public static function fromUrl($url)
    {}

    /**
     * Create a mail attachment from in-memory data.
     *
     * @param  \Closure  $data
     * @param  string|null  $name
     * @return static
     */
    public static function fromData(Closure $data, $name = null)
    {}

    /**
     * Create a mail attachment from an UploadedFile instance.
     *
     * @param  \Illuminate\Http\UploadedFile  $file
     * @return static
     */
    public static function fromUploadedFile(UploadedFile $file)
    {}

    /**
     * Create a mail attachment from a file on the default storage disk.
     *
     * @param  string  $path
     * @return static
     */
    public static function fromStorage($path)
    {}

    /**
     * Create a mail attachment from a file on the specified storage disk.
     *
     * @param  string|null  $disk
     * @param  string  $path
     * @return static
     */
    public static function fromStorageDisk($disk, $path)
    {}

    /**
     * Create a mail attachment from a file on the cloud storage disk.
     *
     * @param  string  $path
     * @return static
     */
    public static function fromCloudStorage($path)
    {}

    /**
     * Set the attached file's filename.
     *
     * @param  string|null  $name
     * @return $this
     */
    public function as($name)
    {}

    /**
     * Set the attached file's MIME type.
     *
     * @param  string  $mime
     * @return $this
     */
    public function withMime($mime)
    {}

    /**
     * Attach the attachment with the given strategies.
     *
     * @param  \Closure  $pathStrategy
     * @param  \Closure  $dataStrategy
     * @return mixed
     */
    public function attachWith(Closure $pathStrategy, Closure $dataStrategy)
    {}

    /**
     * Attach the attachment to a built-in mail type.
     *
     * @param  \Illuminate\Mail\Mailable|\Illuminate\Mail\Message|\Illuminate\Notifications\Messages\MailMessage  $mail
     * @param  array  $options
     * @return mixed
     *
     * @throws \RuntimeException
     */
    public function attachTo($mail, $options = [])
    {}

    /**
     * Determine if the given attachment is equivalent to this attachment.
     *
     * @param  \Illuminate\Mail\Attachment  $attachment
     * @param  array  $options
     * @return bool
     */
    public function isEquivalent(Attachment $attachment, $options = [])
    {}
}
