<?php

namespace Illuminate\Mail;

use Aws\Ses\SesClient;
use Aws\SesV2\SesV2Client;
use Closure;
use Illuminate\Contracts\Mail\Factory as FactoryContract;
use Illuminate\Log\LogManager;
use Illuminate\Mail\Transport\ArrayTransport;
use Illuminate\Mail\Transport\CloudflareTransport;
use Illuminate\Mail\Transport\LogTransport;
use Illuminate\Mail\Transport\ResendTransport;
use Illuminate\Mail\Transport\SesTransport;
use Illuminate\Mail\Transport\SesV2Transport;
use Illuminate\Support\Arr;
use Illuminate\Support\ConfigurationUrlParser;
use Illuminate\Support\Str;
use InvalidArgumentException;
use Psr\Log\LoggerInterface;
use Resend;
use Symfony\Component\HttpClient\HttpClient;
use Symfony\Component\Mailer\Bridge\Mailgun\Transport\MailgunTransportFactory;
use Symfony\Component\Mailer\Bridge\Postmark\Transport\PostmarkTransportFactory;
use Symfony\Component\Mailer\Transport\Dsn;
use Symfony\Component\Mailer\Transport\FailoverTransport;
use Symfony\Component\Mailer\Transport\RoundRobinTransport;
use Symfony\Component\Mailer\Transport\SendmailTransport;
use Symfony\Component\Mailer\Transport\Smtp\EsmtpTransport;
use Symfony\Component\Mailer\Transport\Smtp\EsmtpTransportFactory;
use Symfony\Component\Mailer\Transport\Smtp\Stream\SocketStream;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Mail\Mailer
 */
class MailManager implements FactoryContract
{
    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The array of resolved mailers.
     *
     * @var array
     */
    protected $mailers = [];

    /**
     * The registered custom driver creators.
     *
     * @var array
     */
    protected $customCreators = [];

    /**
     * Create a new Mail manager instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     */
    public function __construct($app)
    {}

    /**
     * Get a mailer instance by name.
     *
     * @param  \UnitEnum|string|null  $name
     * @return \Illuminate\Contracts\Mail\Mailer
     */
    public function mailer($name = null)
    {}

    /**
     * Get a mailer driver instance.
     *
     * @param  \UnitEnum|string|null  $driver
     * @return \Illuminate\Mail\Mailer
     */
    public function driver($driver = null)
    {}

    /**
     * Attempt to get the mailer from the local cache.
     *
     * @param  string  $name
     * @return \Illuminate\Mail\Mailer
     */
    protected function get($name)
    {}

    /**
     * Resolve the given mailer.
     *
     * @param  string  $name
     * @return \Illuminate\Mail\Mailer
     *
     * @throws \InvalidArgumentException
     */
    protected function resolve($name)
    {}

    /**
     * Build a new mailer instance.
     *
     * @param  array  $config
     * @return \Illuminate\Mail\Mailer
     */
    public function build($config)
    {}

    /**
     * Create a new transport instance.
     *
     * @param  array  $config
     * @return \Symfony\Component\Mailer\Transport\TransportInterface
     *
     * @throws \InvalidArgumentException
     */
    public function createSymfonyTransport(array $config)
    {}

    /**
     * Create an instance of the Symfony SMTP Transport driver.
     *
     * @param  array  $config
     * @return \Symfony\Component\Mailer\Transport\Smtp\EsmtpTransport
     */
    protected function createSmtpTransport(array $config)
    {}

    /**
     * Configure the additional SMTP driver options.
     *
     * @param  \Symfony\Component\Mailer\Transport\Smtp\EsmtpTransport  $transport
     * @param  array  $config
     * @return \Symfony\Component\Mailer\Transport\Smtp\EsmtpTransport
     */
    protected function configureSmtpTransport(EsmtpTransport $transport, array $config)
    {}

    /**
     * Create an instance of the Symfony Sendmail Transport driver.
     *
     * @param  array  $config
     * @return \Symfony\Component\Mailer\Transport\SendmailTransport
     */
    protected function createSendmailTransport(array $config)
    {}

    /**
     * Create an instance of the Symfony Amazon SES Transport driver.
     *
     * @param  array  $config
     * @return \Illuminate\Mail\Transport\SesTransport
     */
    protected function createSesTransport(array $config)
    {}

    /**
     * Create an instance of the Symfony Amazon SES V2 Transport driver.
     *
     * @param  array  $config
     * @return \Illuminate\Mail\Transport\SesV2Transport
     */
    protected function createSesV2Transport(array $config)
    {}

    /**
     * Add the SES credentials to the configuration array.
     *
     * @param  array  $config
     * @return array
     */
    protected function addSesCredentials(array $config)
    {}

    /**
     * Create an instance of the Resend Transport driver.
     *
     * @param  array  $config
     * @return \Illuminate\Mail\Transport\ResendTransport
     */
    protected function createResendTransport(array $config)
    {}

    /**
     * Create an instance of the Cloudflare Transport driver.
     *
     * @param  array  $config
     * @return \Illuminate\Mail\Transport\CloudflareTransport
     */
    protected function createCloudflareTransport(array $config)
    {}

    /**
     * Create an instance of the Symfony Mail Transport driver.
     *
     * @return \Symfony\Component\Mailer\Transport\SendmailTransport
     */
    protected function createMailTransport()
    {}

    /**
     * Create an instance of the Symfony Mailgun Transport driver.
     *
     * @param  array  $config
     * @return \Symfony\Component\Mailer\Transport\TransportInterface
     */
    protected function createMailgunTransport(array $config)
    {}

    /**
     * Create an instance of the Symfony Postmark Transport driver.
     *
     * @param  array  $config
     * @return \Symfony\Component\Mailer\Bridge\Postmark\Transport\PostmarkApiTransport
     */
    protected function createPostmarkTransport(array $config)
    {}

    /**
     * Create an instance of the Symfony Failover Transport driver.
     *
     * @param  array  $config
     * @return \Symfony\Component\Mailer\Transport\FailoverTransport
     */
    protected function createFailoverTransport(array $config)
    {}

    /**
     * Create an instance of the Symfony Roundrobin Transport driver.
     *
     * @param  array  $config
     * @return \Symfony\Component\Mailer\Transport\RoundRobinTransport
     */
    protected function createRoundrobinTransport(array $config)
    {}

    /**
     * Create an instance of supplied class extending the Symfony Roundrobin Transport driver.
     *
     * @template TClass of \Symfony\Component\Mailer\Transport\RoundRobinTransport
     *
     * @param  array  $config
     * @param  class-string<TClass>  $class
     * @return TClass
     *
     * @throws \InvalidArgumentException
     */
    protected function createRoundrobinTransportOfClass(array $config, string $class)
    {}

    /**
     * Create an instance of the Log Transport driver.
     *
     * @param  array  $config
     * @return \Illuminate\Mail\Transport\LogTransport
     */
    protected function createLogTransport(array $config)
    {}

    /**
     * Create an instance of the Array Transport Driver.
     *
     * @return \Illuminate\Mail\Transport\ArrayTransport
     */
    protected function createArrayTransport()
    {}

    /**
     * Get a configured Symfony HTTP client instance.
     *
     * @return \Symfony\Contracts\HttpClient\HttpClientInterface|null
     */
    protected function getHttpClient(array $config)
    {}

    /**
     * Set a global address on the mailer by type.
     *
     * @param  \Illuminate\Mail\Mailer  $mailer
     * @param  array  $config
     * @param  string  $type
     * @return void
     */
    protected function setGlobalAddress($mailer, array $config, string $type)
    {}

    /**
     * Get the mail connection configuration.
     *
     * @param  string  $name
     * @return array
     */
    protected function getConfig(string $name)
    {}

    /**
     * Get the default mail driver name.
     *
     * @return string
     */
    public function getDefaultDriver()
    {}

    /**
     * Set the default mail driver name.
     *
     * @param  \UnitEnum|string  $name
     * @return void
     */
    public function setDefaultDriver($name)
    {}

    /**
     * Disconnect the given mailer and remove from local cache.
     *
     * @param  \UnitEnum|string|null  $name
     * @return void
     */
    public function purge($name = null)
    {}

    /**
     * Register a custom transport creator Closure.
     *
     * @param  string  $driver
     * @param  \Closure  $callback
     * @return $this
     */
    public function extend($driver, Closure $callback)
    {}

    /**
     * Get the application instance used by the manager.
     *
     * @return \Illuminate\Contracts\Foundation\Application
     */
    public function getApplication()
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
     * Forget all of the resolved mailer instances.
     *
     * @return $this
     */
    public function forgetMailers()
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
