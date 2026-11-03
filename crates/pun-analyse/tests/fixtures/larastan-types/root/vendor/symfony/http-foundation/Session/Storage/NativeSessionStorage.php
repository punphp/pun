<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation\Session\Storage;

use Symfony\Component\HttpFoundation\Session\SessionBagInterface;
use Symfony\Component\HttpFoundation\Session\Storage\Handler\StrictSessionHandler;
use Symfony\Component\HttpFoundation\Session\Storage\Proxy\AbstractProxy;
use Symfony\Component\HttpFoundation\Session\Storage\Proxy\SessionHandlerProxy;

// Help opcache.preload discover always-needed symbols
class_exists(MetadataBag::class);
class_exists(StrictSessionHandler::class);
class_exists(SessionHandlerProxy::class);

/**
 * This provides a base class for session attribute storage.
 *
 * @author Drak <drak@zikula.org>
 */
class NativeSessionStorage implements SessionStorageInterface
{
    /**
     * @var SessionBagInterface[]
     */
    protected array $bags = [];
    protected bool $started = false;
    protected bool $closed = false;
    protected AbstractProxy|\SessionHandlerInterface $saveHandler;
    protected MetadataBag $metadataBag;

    private static bool $shutdownRegistered = false;

    /**
     * Depending on how you want the storage driver to behave you probably
     * want to override this constructor entirely.
     *
     * List of options for $options array with their defaults.
     *
     * @see https://php.net/session.configuration for options
     * but we omit 'session.' from the beginning of the keys for convenience.
     *
     * ("auto_start", is not supported as it tells PHP to start a session before
     * PHP starts to execute user-land code. Setting during runtime has no effect).
     *
     * cache_limiter, "" (use "0" to prevent headers from being sent entirely).
     * cache_expire, "0"
     * cookie_domain, ""
     * cookie_httponly, ""
     * cookie_lifetime, "0"
     * cookie_path, "/"
     * cookie_secure, ""
     * cookie_samesite, null
     * gc_divisor, "100"
     * gc_maxlifetime, "1440"
     * gc_probability, "1"
     * lazy_write, "1"
     * name, "PHPSESSID"
     * serialize_handler, "php"
     * use_strict_mode, "1"
     * use_cookies, "1"
     */
    public function __construct(array $options = [], AbstractProxy|\SessionHandlerInterface|null $handler = null, ?MetadataBag $metaBag = null)
    {}

    /**
     * Gets the save handler instance.
     */
    public function getSaveHandler(): AbstractProxy|\SessionHandlerInterface
    {}

    public function start(): bool
    {}

    public function getId(): string
    {}

    public function setId(string $id): void
    {}

    public function getName(): string
    {}

    public function setName(string $name): void
    {}

    public function regenerate(bool $destroy = false, ?int $lifetime = null): bool
    {}

    public function save(): void
    {}

    public function clear(): void
    {}

    public function registerBag(SessionBagInterface $bag): void
    {}

    public function getBag(string $name): SessionBagInterface
    {}

    public function setMetadataBag(?MetadataBag $metaBag): void
    {}

    /**
     * Gets the MetadataBag.
     */
    public function getMetadataBag(): MetadataBag
    {}

    public function isStarted(): bool
    {}

    /**
     * Sets session.* ini variables.
     *
     * For convenience we omit 'session.' from the beginning of the keys.
     * Explicitly ignores other ini keys.
     *
     * @param array $options Session ini directives [key => value]
     *
     * @see https://php.net/session.configuration
     */
    public function setOptions(array $options): void
    {}

    /**
     * Registers session save handler as a PHP session handler.
     *
     * To use internal PHP session save handlers, override this method using ini_set with
     * session.save_handler and session.save_path e.g.
     *
     *     ini_set('session.save_handler', 'files');
     *     ini_set('session.save_path', '/tmp');
     *
     * or pass in a \SessionHandler instance which configures session.save_handler in the
     * constructor, for a template see NativeFileSessionHandler.
     *
     * @see https://php.net/session-set-save-handler
     * @see https://php.net/sessionhandlerinterface
     * @see https://php.net/sessionhandler
     *
     * @throws \InvalidArgumentException
     */
    public function setSaveHandler(AbstractProxy|\SessionHandlerInterface|null $saveHandler): void
    {}

    /**
     * Load the session with attributes.
     *
     * After starting the session, PHP retrieves the session from whatever handlers
     * are set to (either PHP's internal, or a custom save handler set with session_set_save_handler()).
     * PHP takes the return value from the read() handler, unserializes it
     * and populates $_SESSION with the result automatically.
     */
    protected function loadSession(?array &$session = null): void
    {}
}
