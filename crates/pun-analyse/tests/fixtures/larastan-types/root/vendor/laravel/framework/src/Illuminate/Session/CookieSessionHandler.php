<?php

namespace Illuminate\Session;

use Illuminate\Contracts\Cookie\QueueingFactory as CookieJar;
use Illuminate\Support\InteractsWithTime;
use RuntimeException;
use SessionHandlerInterface;
use Symfony\Component\HttpFoundation\Request;

class CookieSessionHandler implements SessionHandlerInterface
{
    use InteractsWithTime;

    /**
     * The cookie jar instance.
     *
     * @var \Illuminate\Contracts\Cookie\Factory
     */
    protected $cookie;

    /**
     * The request instance.
     *
     * @var \Symfony\Component\HttpFoundation\Request
     */
    protected $request;

    /**
     * The number of minutes the session should be valid.
     *
     * @var int
     */
    protected $minutes;

    /**
     * Indicates whether the session should be expired when the browser closes.
     *
     * @var bool
     */
    protected $expireOnClose;

    /**
     * Create a new cookie driven handler instance.
     *
     * @param  \Illuminate\Contracts\Cookie\QueueingFactory  $cookie
     * @param  int  $minutes
     * @param  bool  $expireOnClose
     */
    public function __construct(CookieJar $cookie, $minutes, $expireOnClose = false)
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    public function open($savePath, $sessionName): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    public function close(): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @return string
     */
    public function create_sid(): string
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    public function validateId($id): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @return string|false
     */
    public function read($sessionId): string|false
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    public function write($sessionId, $data): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    public function destroy($sessionId): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @return int
     */
    public function gc($lifetime): int
    {}

    /**
     * Set the request instance.
     *
     * @param  \Symfony\Component\HttpFoundation\Request  $request
     * @return void
     */
    public function setRequest(Request $request)
    {}
}
