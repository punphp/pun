<?php

declare(strict_types=1);

namespace GuzzleHttp;

use GuzzleHttp\Promise\PromiseInterface;
use GuzzleHttp\Psr7\DiagnosticValue;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;

/**
 * Creates a composed Guzzle handler function by stacking middlewares on top of
 * an HTTP handler function.
 *
 * @template THandler
 *
 * @final
 */
class HandlerStack
{
    use NonSerializableTrait;

    /**
     * @var (callable&THandler)|null
     */
    private $handler;

    /**
     * @var array<int, array{0: callable(callable&THandler): (callable&THandler), 1: string|null}>
     */
    private array $stack = [];

    /**
     * @var (callable&THandler)|null
     */
    private $cached;

    /**
     * Creates a default handler stack that can be used by clients.
     *
     * The returned handler will wrap the provided handler or use the most
     * appropriate default handler for your system. The returned HandlerStack
     * has support for authentication, cookies, redirects, HTTP error
     * exceptions, and preparing a body before sending.
     *
     * The returned handler stack can be passed to a client in the "handler"
     * option.
     *
     * @param (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)|null $handler HTTP handler function to use with the stack. If no
     *                                                                                                                        handler is provided, the best handler for your
     *                                                                                                                        system will be utilized.
     *
     * @return self<callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>>
     */
    public static function create(?callable $handler = null): self
    {}

    /**
     * @param (callable&THandler)|null $handler Underlying handler.
     */
    public function __construct(?callable $handler = null)
    {}

    /**
     * Invokes the handler stack as a composed handler
     *
     * @return PromiseInterface<ResponseInterface, mixed>
     */
    public function __invoke(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        array $options
    ) {}

    /**
     * Set the HTTP handler that actually returns a promise.
     *
     * @param callable&THandler $handler Accepts a request and array of options and returns a value expected by the stack.
     */
    public function setHandler(callable $handler): void
    {}

    /**
     * Returns true if the builder has a handler.
     */
    public function hasHandler(): bool
    {}

    /**
     * Unshift a middleware to the bottom of the stack.
     *
     * @param callable(callable&THandler): (callable&THandler) $middleware Middleware function
     * @param string                                           $name       Name to register for this middleware.
     */
    public function unshift(callable $middleware, ?string $name = null): void
    {}

    /**
     * Push a middleware to the top of the stack.
     *
     * @param callable(callable&THandler): (callable&THandler) $middleware Middleware function
     * @param string                                           $name       Name to register for this middleware.
     */
    public function push(callable $middleware, string $name = ''): void
    {}

    /**
     * Add a middleware before another middleware by name.
     *
     * @param string                                           $findName   Middleware to find
     * @param callable(callable&THandler): (callable&THandler) $middleware Middleware function
     * @param string                                           $withName   Name to register for this middleware.
     */
    public function before(string $findName, callable $middleware, string $withName = ''): void
    {}

    /**
     * Add a middleware after another middleware by name.
     *
     * @param string                                           $findName   Middleware to find
     * @param callable(callable&THandler): (callable&THandler) $middleware Middleware function
     * @param string                                           $withName   Name to register for this middleware.
     */
    public function after(string $findName, callable $middleware, string $withName = ''): void
    {}

    /**
     * Remove a middleware by instance or name from the stack.
     *
     * @param (callable(callable&THandler): (callable&THandler))|string $remove Middleware to remove by instance or name.
     */
    public function remove($remove): void
    {}

    /**
     * Compose the middleware and handler into a single callable function.
     *
     * @return callable&THandler
     */
    public function resolve(): callable
    {}

    public function __unserialize(array $data): void
    {}

    private function findByName(string $name): int
    {}

    /**
     * Splices a function into the middleware list at a specific position.
     *
     * @param callable(callable&THandler): (callable&THandler) $middleware
     */
    private function splice(string $findName, string $withName, callable $middleware, bool $before): void
    {}
}
