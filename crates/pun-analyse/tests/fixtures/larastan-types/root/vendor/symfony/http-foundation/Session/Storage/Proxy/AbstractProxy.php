<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation\Session\Storage\Proxy;

/**
 * @author Drak <drak@zikula.org>
 */
abstract class AbstractProxy
{
    protected bool $wrapper = false;

    protected ?string $saveHandlerName = null;

    /**
     * Gets the session.save_handler name.
     */
    public function getSaveHandlerName(): ?string
    {}

    /**
     * Is this proxy handler and instance of \SessionHandlerInterface.
     */
    public function isSessionHandlerInterface(): bool
    {}

    /**
     * Returns true if this handler wraps an internal PHP session save handler using \SessionHandler.
     */
    public function isWrapper(): bool
    {}

    /**
     * Has a session started?
     */
    public function isActive(): bool
    {}

    /**
     * Gets the session ID.
     */
    public function getId(): string
    {}

    /**
     * Sets the session ID.
     *
     * @throws \LogicException
     */
    public function setId(string $id): void
    {}

    /**
     * Gets the session name.
     */
    public function getName(): string
    {}

    /**
     * Sets the session name.
     *
     * @throws \LogicException
     */
    public function setName(string $name): void
    {}
}
