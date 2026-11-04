<?php

namespace Illuminate\Filesystem;

use Closure;
use Illuminate\Container\Container;
use Illuminate\Contracts\Debug\ExceptionHandler;
use Illuminate\Contracts\Filesystem\Cloud as CloudFilesystemContract;
use Illuminate\Contracts\Filesystem\Factory as FilesystemFactory;
use Illuminate\Contracts\Filesystem\Filesystem as FilesystemContract;
use Illuminate\Http\File;
use Illuminate\Http\Request;
use Illuminate\Http\UploadedFile;
use Illuminate\Image\Image;
use Illuminate\Support\Arr;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use League\Flysystem\FilesystemAdapter as FlysystemAdapter;
use League\Flysystem\FilesystemOperator;
use League\Flysystem\Ftp\FtpAdapter;
use League\Flysystem\Local\LocalFilesystemAdapter as LocalAdapter;
use League\Flysystem\PathPrefixer;
use League\Flysystem\PhpseclibV3\SftpAdapter;
use League\Flysystem\StorageAttributes;
use League\Flysystem\UnableToCopyFile;
use League\Flysystem\UnableToCreateDirectory;
use League\Flysystem\UnableToDeleteDirectory;
use League\Flysystem\UnableToDeleteFile;
use League\Flysystem\UnableToMoveFile;
use League\Flysystem\UnableToProvideChecksum;
use League\Flysystem\UnableToReadFile;
use League\Flysystem\UnableToRetrieveMetadata;
use League\Flysystem\UnableToSetVisibility;
use League\Flysystem\UnableToWriteFile;
use League\Flysystem\Visibility;
use League\Flysystem\WhitespacePathNormalizer;
use PHPUnit\Framework\Assert as PHPUnit;
use Psr\Http\Message\StreamInterface;
use RuntimeException;
use Symfony\Component\HttpFoundation\StreamedResponse;

/**
 * @mixin \League\Flysystem\FilesystemOperator
 */
class FilesystemAdapter implements CloudFilesystemContract
{
    use Conditionable;
    use Macroable {
        __call as macroCall;
    }

    /**
     * The Flysystem filesystem implementation.
     *
     * @var \League\Flysystem\FilesystemOperator
     */
    protected $driver;

    /**
     * The Flysystem adapter implementation.
     *
     * @var \League\Flysystem\FilesystemAdapter
     */
    protected $adapter;

    /**
     * The filesystem configuration.
     *
     * @var array
     */
    protected $config;

    /**
     * The Flysystem PathPrefixer instance.
     *
     * @var \League\Flysystem\PathPrefixer
     */
    protected $prefixer;

    /**
     * The file server callback.
     *
     * @var \Closure|null
     */
    protected $serveCallback;

    /**
     * The temporary URL builder callback.
     *
     * @var \Closure|null
     */
    protected $temporaryUrlCallback;

    /**
     * The temporary upload URL builder callback.
     *
     * @var \Closure|null
     */
    protected $temporaryUploadUrlCallback;

    /**
     * Create a new filesystem adapter instance.
     *
     * @param  \League\Flysystem\FilesystemOperator  $driver
     * @param  \League\Flysystem\FilesystemAdapter  $adapter
     * @param  array  $config
     */
    public function __construct(FilesystemOperator $driver, FlysystemAdapter $adapter, array $config = [])
    {}

    /**
     * Assert that the given file or directory exists.
     *
     * @param  string|array  $path
     * @param  string|null  $content
     * @return $this
     */
    public function assertExists($path, $content = null)
    {}

    /**
     * Assert that the number of files in path equals the expected count.
     *
     * @param  string  $path
     * @param  int  $count
     * @param  bool  $recursive
     * @return $this
     */
    public function assertCount($path, $count, $recursive = false)
    {}

    /**
     * Assert that the given file or directory does not exist.
     *
     * @param  string|array  $path
     * @return $this
     */
    public function assertMissing($path)
    {}

    /**
     * Assert that the given directory is empty.
     *
     * @param  string  $path
     * @return $this
     */
    public function assertDirectoryEmpty($path)
    {}

    /**
     * Assert that the disk contains no files.
     *
     * @return $this
     */
    public function assertEmpty()
    {}

    /**
     * Determine if a file or directory exists.
     *
     * @param  string  $path
     * @return bool
     */
    public function exists($path)
    {}

    /**
     * Determine if a file or directory is missing.
     *
     * @param  string  $path
     * @return bool
     */
    public function missing($path)
    {}

    /**
     * Determine if a file exists.
     *
     * @param  string  $path
     * @return bool
     */
    public function fileExists($path)
    {}

    /**
     * Determine if a file is missing.
     *
     * @param  string  $path
     * @return bool
     */
    public function fileMissing($path)
    {}

    /**
     * Determine if a directory exists.
     *
     * @param  string  $path
     * @return bool
     */
    public function directoryExists($path)
    {}

    /**
     * Determine if a directory is missing.
     *
     * @param  string  $path
     * @return bool
     */
    public function directoryMissing($path)
    {}

    /**
     * Get the full path to the file that exists at the given relative path.
     *
     * @param  string  $path
     * @return string
     */
    public function path($path)
    {}

    /**
     * Get the contents of a file.
     *
     * @param  string  $path
     * @return string|null
     */
    public function get($path)
    {}

    /**
     * Get the contents of a file as decoded JSON.
     *
     * @param  string  $path
     * @param  int  $flags
     * @return array|null
     */
    public function json($path, $flags = 0)
    {}

    /**
     * Create a streamed response for a given file.
     *
     * @param  string  $path
     * @param  string|null  $name
     * @param  array  $headers
     * @param  string|null  $disposition
     * @return \Symfony\Component\HttpFoundation\StreamedResponse
     *
     * @throws UnableToRetrieveMetadata
     */
    public function response($path, $name = null, array $headers = [], $disposition = 'inline')
    {}

    /**
     * Create a streamed download response for a given file.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  string  $path
     * @param  string|null  $name
     * @param  array  $headers
     * @return \Symfony\Component\HttpFoundation\StreamedResponse
     *
     * @throws UnableToRetrieveMetadata
     */
    public function serve(Request $request, $path, $name = null, array $headers = [])
    {}

    /**
     * Create a streamed download response for a given file.
     *
     * @param  string  $path
     * @param  string|null  $name
     * @param  array  $headers
     * @return \Symfony\Component\HttpFoundation\StreamedResponse
     *
     * @throws UnableToRetrieveMetadata
     */
    public function download($path, $name = null, array $headers = [])
    {}

    /**
     * Create an image instance from a file in storage.
     */
    public function image(string $path): Image
    {}

    /**
     * Convert the string to ASCII characters that are equivalent to the given name.
     *
     * @param  string  $name
     * @return string
     */
    protected function fallbackName($name)
    {}

    /**
     * Write the contents of a file.
     *
     * @param  string  $path
     * @param  \Psr\Http\Message\StreamInterface|\Illuminate\Http\File|\Illuminate\Http\UploadedFile|string|resource  $contents
     * @param  mixed  $options
     * @return string|bool
     */
    public function put($path, $contents, $options = [])
    {}

    /**
     * Store the uploaded file on the disk.
     *
     * @param  \Illuminate\Http\File|\Illuminate\Http\UploadedFile|string  $path
     * @param  \Illuminate\Http\File|\Illuminate\Http\UploadedFile|string|array|null  $file
     * @param  mixed  $options
     * @return string|false
     */
    public function putFile($path, $file = null, $options = [])
    {}

    /**
     * Store the uploaded file on the disk with a given name.
     *
     * @param  \Illuminate\Http\File|\Illuminate\Http\UploadedFile|string  $path
     * @param  \Illuminate\Http\File|\Illuminate\Http\UploadedFile|string|array|null  $file
     * @param  string|array|null  $name
     * @param  mixed  $options
     * @return string|false
     */
    public function putFileAs($path, $file, $name = null, $options = [])
    {}

    /**
     * Get the visibility for the given path.
     *
     * @param  string  $path
     * @return string
     */
    public function getVisibility($path)
    {}

    /**
     * Set the visibility for the given path.
     *
     * @param  string  $path
     * @param  string  $visibility
     * @return bool
     */
    public function setVisibility($path, $visibility)
    {}

    /**
     * Prepend to a file.
     *
     * @param  string  $path
     * @param  string  $data
     * @param  string  $separator
     * @return bool
     */
    public function prepend($path, $data, $separator = PHP_EOL)
    {}

    /**
     * Append to a file.
     *
     * @param  string  $path
     * @param  string  $data
     * @param  string  $separator
     * @return bool
     */
    public function append($path, $data, $separator = PHP_EOL)
    {}

    /**
     * Delete the file at a given path.
     *
     * @param  string|array  $paths
     * @return bool
     */
    public function delete($paths)
    { func_get_args(); }

    /**
     * Copy a file to a new location.
     *
     * @param  string  $from
     * @param  string  $to
     * @return bool
     */
    public function copy($from, $to)
    {}

    /**
     * Move a file to a new location.
     *
     * @param  string  $from
     * @param  string  $to
     * @return bool
     */
    public function move($from, $to)
    {}

    /**
     * Copy a file to another disk.
     *
     * @param  \UnitEnum|string|\Illuminate\Contracts\Filesystem\Filesystem  $disk
     * @param  string  $from
     * @param  string|null  $to
     * @return bool
     */
    public function copyToDisk($disk, $from, $to = null)
    {}

    /**
     * Move a file to another disk.
     *
     * @param  \UnitEnum|string|\Illuminate\Contracts\Filesystem\Filesystem  $disk
     * @param  string  $from
     * @param  string|null  $to
     * @return bool
     */
    public function moveToDisk($disk, $from, $to = null)
    {}

    /**
     * Get the file size of a given file.
     *
     * @param  string  $path
     * @return int
     */
    public function size($path)
    {}

    /**
     * Get the checksum for a file.
     *
     * @return string|false
     *
     * @throws UnableToProvideChecksum
     */
    public function checksum(string $path, array $options = [])
    {}

    /**
     * Get the mime-type of a given file.
     *
     * @param  string  $path
     * @return string|false
     *
     * @throws UnableToRetrieveMetadata
     */
    public function mimeType($path)
    {}

    /**
     * Get the file's last modification time.
     *
     * @param  string  $path
     * @return int
     */
    public function lastModified($path)
    {}

    /**
     * {@inheritdoc}
     */
    public function readStream($path)
    {}

    /**
     * {@inheritdoc}
     */
    public function writeStream($path, $resource, array $options = [])
    {}

    /**
     * Get the URL for the file at the given path.
     *
     * @param  string  $path
     * @return string
     *
     * @throws \RuntimeException
     */
    public function url($path)
    {}

    /**
     * Get the URL for the file at the given path.
     *
     * @param  string  $path
     * @return string
     */
    protected function getFtpUrl($path)
    {}

    /**
     * Get the URL for the file at the given path.
     *
     * @param  string  $path
     * @return string
     */
    protected function getLocalUrl($path)
    {}

    /**
     * Determine if temporary URLs can be generated.
     *
     * @return bool
     */
    public function providesTemporaryUrls()
    {}

    /**
     * Determine if temporary upload URLs can be generated.
     *
     * @return bool
     */
    public function providesTemporaryUploadUrls()
    {}

    /**
     * Get a temporary URL for the file at the given path.
     *
     * @param  string  $path
     * @param  \DateTimeInterface  $expiration
     * @param  array  $options
     * @return string
     *
     * @throws \RuntimeException
     */
    public function temporaryUrl($path, $expiration, array $options = [])
    {}

    /**
     * Get a temporary upload URL for the file at the given path.
     *
     * @param  string  $path
     * @param  \DateTimeInterface  $expiration
     * @param  array  $options
     * @return array
     *
     * @throws \RuntimeException
     */
    public function temporaryUploadUrl($path, $expiration, array $options = [])
    {}

    /**
     * Concatenate a path to a URL.
     *
     * @param  string  $url
     * @param  string  $path
     * @return string
     */
    protected function concatPathToUrl($url, $path)
    {}

    /**
     * Replace the scheme, host and port of the given UriInterface with values from the given URL.
     *
     * @param  \Psr\Http\Message\UriInterface  $uri
     * @param  string  $url
     * @return \Psr\Http\Message\UriInterface
     */
    protected function replaceBaseUrl($uri, $url)
    {}

    /**
     * Get an array of all files in a directory.
     *
     * @param  string|null  $directory
     * @param  bool  $recursive
     * @return array<string>
     */
    public function files($directory = null, $recursive = false)
    {}

    /**
     * Get all of the files from the given directory (recursive).
     *
     * @param  string|null  $directory
     * @return array<string>
     */
    public function allFiles($directory = null)
    {}

    /**
     * Get all of the directories within a given directory.
     *
     * @param  string|null  $directory
     * @param  bool  $recursive
     * @return array<string>
     */
    public function directories($directory = null, $recursive = false)
    {}

    /**
     * Get all the directories within a given directory (recursive).
     *
     * @param  string|null  $directory
     * @return array<string>
     */
    public function allDirectories($directory = null)
    {}

    /**
     * Create a directory.
     *
     * @param  string  $path
     * @return bool
     */
    public function makeDirectory($path)
    {}

    /**
     * Recursively delete a directory.
     *
     * @param  string  $directory
     * @return bool
     */
    public function deleteDirectory($directory)
    {}

    /**
     * Get the Flysystem driver.
     *
     * @return \League\Flysystem\FilesystemOperator
     */
    public function getDriver()
    {}

    /**
     * Get the Flysystem adapter.
     *
     * @return \League\Flysystem\FilesystemAdapter
     */
    public function getAdapter()
    {}

    /**
     * Get the configuration values.
     *
     * @return array
     */
    public function getConfig()
    {}

    /**
     * Parse the given visibility value.
     *
     * @param  string|null  $visibility
     * @return string|null
     *
     * @throws \InvalidArgumentException
     */
    protected function parseVisibility($visibility)
    {}

    /**
     * Define a custom callback that generates file download responses.
     *
     * @param  \Closure  $callback
     * @return void
     */
    public function serveUsing(Closure $callback)
    {}

    /**
     * Define a custom temporary URL builder callback.
     *
     * @param  \Closure  $callback
     * @return void
     */
    public function buildTemporaryUrlsUsing(Closure $callback)
    {}

    /**
     * Define a custom temporary upload URL builder callback.
     *
     * @param  \Closure  $callback
     * @return void
     */
    public function buildTemporaryUploadUrlsUsing(Closure $callback)
    {}

    /**
     * Determine if Flysystem exceptions should be thrown.
     *
     * @return bool
     */
    protected function throwsExceptions(): bool
    {}

    /**
     * Report the exception.
     *
     * @param  \Throwable  $exception
     * @return void
     *
     * @throws \Throwable
     */
    protected function report($exception)
    {}

    /**
     * Determine if Flysystem exceptions should be reported.
     *
     * @return bool
     */
    protected function shouldReport(): bool
    {}

    /**
     * Pass dynamic methods call onto Flysystem.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     *
     * @throws \BadMethodCallException
     */
    public function __call($method, $parameters)
    {}
}
