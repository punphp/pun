<?php

namespace Illuminate\Filesystem;

use Aws\Credentials\CredentialProvider;
use Aws\S3\S3Client;
use Closure;
use Illuminate\Contracts\Filesystem\Factory as FactoryContract;
use Illuminate\Support\Arr;
use Illuminate\Support\RebindsCallbacksToSelf;
use Illuminate\Support\Str;
use InvalidArgumentException;
use League\Flysystem\AwsS3V3\AwsS3V3Adapter as S3Adapter;
use League\Flysystem\AwsS3V3\PortableVisibilityConverter as AwsS3PortableVisibilityConverter;
use League\Flysystem\Filesystem as Flysystem;
use League\Flysystem\FilesystemAdapter as FlysystemAdapter;
use League\Flysystem\Ftp\FtpAdapter;
use League\Flysystem\Ftp\FtpConnectionOptions;
use League\Flysystem\Local\LocalFilesystemAdapter as LocalAdapter;
use League\Flysystem\PathPrefixing\PathPrefixedAdapter;
use League\Flysystem\PhpseclibV3\SftpAdapter;
use League\Flysystem\PhpseclibV3\SftpConnectionProvider;
use League\Flysystem\ReadOnly\ReadOnlyFilesystemAdapter;
use League\Flysystem\UnixVisibility\PortableVisibilityConverter;
use League\Flysystem\Visibility;
use ReflectionException;
use RuntimeException;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Contracts\Filesystem\Filesystem
 * @mixin \Illuminate\Filesystem\FilesystemAdapter
 */
class FilesystemManager implements FactoryContract
{
    use RebindsCallbacksToSelf;

    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The array of resolved filesystem drivers.
     *
     * @var array
     */
    protected $disks = [];

    /**
     * The registered custom driver creators.
     *
     * @var array
     */
    protected $customCreators = [];

    /**
     * Create a new filesystem manager instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     */
    public function __construct($app)
    {}

    /**
     * Get a filesystem instance.
     *
     * @param  \UnitEnum|string|null  $name
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     */
    public function drive($name = null)
    {}

    /**
     * Get a filesystem instance.
     *
     * @param  \UnitEnum|string|null  $name
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     */
    public function disk($name = null)
    {}

    /**
     * Get a default cloud filesystem instance.
     *
     * @return \Illuminate\Contracts\Filesystem\Cloud
     */
    public function cloud()
    {}

    /**
     * Build an on-demand disk.
     *
     * @param  string|array  $config
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     */
    public function build($config)
    {}

    /**
     * Attempt to get the disk from the local cache.
     *
     * @param  string  $name
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     */
    protected function get($name)
    {}

    /**
     * Resolve the given disk.
     *
     * @param  string  $name
     * @param  array|null  $config
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     *
     * @throws \InvalidArgumentException
     */
    protected function resolve($name, $config = null)
    {}

    /**
     * Call a custom driver creator.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     */
    protected function callCustomCreator(array $config)
    {}

    /**
     * Create an instance of the local driver.
     *
     * @param  array  $config
     * @param  string  $name
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     */
    public function createLocalDriver(array $config, string $name = 'local')
    {}

    /**
     * Create an instance of the ftp driver.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     */
    public function createFtpDriver(array $config)
    {}

    /**
     * Create an instance of the sftp driver.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     */
    public function createSftpDriver(array $config)
    {}

    /**
     * Create an instance of the Amazon S3 driver.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Filesystem\Cloud
     */
    public function createS3Driver(array $config)
    {}

    /**
     * Create a read-through filesystem driver.
     *
     * @param  array  $config
     * @param  string  $name
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     */
    public function createReadThroughDriver(array $config, string $name = 'read-through')
    {}

    /**
     * Format the given S3 configuration with the default options.
     *
     * @param  array  $config
     * @return array
     *
     * @throws \InvalidArgumentException
     */
    protected function formatS3Config(array $config)
    {}

    /**
     * Create a scoped driver.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Filesystem\Filesystem
     *
     * @throws \InvalidArgumentException
     */
    public function createScopedDriver(array $config)
    {}

    /**
     * Create a Flysystem instance with the given adapter.
     *
     * @param  \League\Flysystem\FilesystemAdapter  $adapter
     * @param  array  $config
     * @return \League\Flysystem\FilesystemOperator
     */
    protected function createFlysystem(FlysystemAdapter $adapter, array $config)
    {}

    /**
     * Set the given disk instance.
     *
     * @param  \UnitEnum|string  $name
     * @param  mixed  $disk
     * @return $this
     */
    public function set($name, $disk)
    {}

    /**
     * Get the filesystem connection configuration.
     *
     * @param  string  $name
     * @return array
     */
    protected function getConfig($name)
    {}

    /**
     * Get the default driver name.
     *
     * @return string
     */
    public function getDefaultDriver()
    {}

    /**
     * Get the default cloud driver name.
     *
     * @return string
     */
    public function getDefaultCloudDriver()
    {}

    /**
     * Unset the given disk instances.
     *
     * @param  array<\UnitEnum|string>|\UnitEnum|string  $disk
     * @return $this
     */
    public function forgetDisk($disk)
    {}

    /**
     * Disconnect the given disk and remove from local cache.
     *
     * @param  \UnitEnum|string|null  $name
     * @return void
     */
    public function purge($name = null)
    {}

    /**
     * Register a custom driver creator Closure.
     *
     * @param  string  $driver
     * @param  \Closure  $callback
     *
     * @param-closure-this  $this  $callback
     *
     * @return $this
     */
    public function extend($driver, Closure $callback)
    {}

    /**
     * Set the application instance used by the manager.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     * @return $this
     */
    public function setApplication($app)
    {}

    /**
     * Dynamically call the default driver instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
