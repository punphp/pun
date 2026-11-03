<?php

declare(strict_types=1);

namespace GuzzleHttp\Handler;

use GuzzleHttp\Exception\InvalidArgumentException;
use GuzzleHttp\NonSerializableTrait;
use GuzzleHttp\Promise\PromiseInterface;
use GuzzleHttp\Psr7\DiagnosticValue;
use GuzzleHttp\TransportSharing;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;

/**
 * HTTP handler that uses cURL easy handles as a transport layer.
 *
 * When using the CurlHandler, custom curl options can be specified as an
 * associative array of curl option constants mapping to values in the
 * **curl** key of the "client" key of the request.
 */
final class CurlHandler
{
    use NonSerializableTrait;

    private const KNOWN_CONSTRUCTOR_OPTIONS = [
        'handle_factory' => true,
        'transport_sharing' => true,
    ];

    private CurlFactoryInterface $factory;

    private bool $ownsFactory;

    private bool $closed = false;

    private ?CurlShareHandleState $shareHandleState;

    /**
     * Accepts an associative array of options:
     *
     * - handle_factory: Optional curl factory used to create cURL handles.
     * - transport_sharing: Optional transport sharing mode.
     *
     * @param array{handle_factory?: ?CurlFactoryInterface, transport_sharing?: mixed} $options Array of options to use with the handler
     */
    public function __construct(array $options = [])
    {}

    /**
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    public function __invoke(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ): PromiseInterface {}

    /**
     * Closes native cURL resources owned by this handler.
     *
     * After closing, the handler is terminal and must not be reused.
     */
    public function close(): void
    {}

    public function __destruct()
    {}

    public function __unserialize(array $data): void
    {}

    private function assertOpen(): void
    {}

    private function doClose(bool $explicit): void
    {}
}
