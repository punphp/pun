<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation\Session;

use Symfony\Component\HttpFoundation\Session\Attribute\AttributeBag;
use Symfony\Component\HttpFoundation\Session\Attribute\AttributeBagInterface;
use Symfony\Component\HttpFoundation\Session\Flash\FlashBag;
use Symfony\Component\HttpFoundation\Session\Flash\FlashBagInterface;
use Symfony\Component\HttpFoundation\Session\Storage\MetadataBag;
use Symfony\Component\HttpFoundation\Session\Storage\NativeSessionStorage;
use Symfony\Component\HttpFoundation\Session\Storage\SessionStorageInterface;

// Help opcache.preload discover always-needed symbols
class_exists(AttributeBag::class);
class_exists(FlashBag::class);
class_exists(SessionBagProxy::class);

/**
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Drak <drak@zikula.org>
 *
 * @implements \IteratorAggregate<string, mixed>
 */
class Session implements FlashBagAwareSessionInterface, \IteratorAggregate, \Countable
{
    protected SessionStorageInterface $storage;

    private string $flashName;
    private string $attributeName;
    private array $data = [];
    private int $usageIndex = 0;
    private ?\Closure $usageReporter;

    public function __construct(?SessionStorageInterface $storage = null, ?AttributeBagInterface $attributes = null, ?FlashBagInterface $flashes = null, ?callable $usageReporter = null)
    {}

    public function start(): bool
    {}

    public function has(string $name): bool
    {}

    public function get(string $name, mixed $default = null): mixed
    {}

    public function set(string $name, mixed $value): void
    {}

    public function all(): array
    {}

    public function replace(array $attributes): void
    {}

    public function remove(string $name): mixed
    {}

    public function clear(): void
    {}

    public function isStarted(): bool
    {}

    /**
     * Returns an iterator for attributes.
     *
     * @return \ArrayIterator<string, mixed>
     */
    public function getIterator(): \ArrayIterator
    {}

    /**
     * Returns the number of attributes.
     */
    public function count(): int
    {}

    public function &getUsageIndex(): int
    {}

    /**
     * @internal
     */
    public function isEmpty(): bool
    {}

    public function invalidate(?int $lifetime = null): bool
    {}

    public function migrate(bool $destroy = false, ?int $lifetime = null): bool
    {}

    public function save(): void
    {}

    public function getId(): string
    {}

    public function setId(string $id): void
    {}

    public function getName(): string
    {}

    public function setName(string $name): void
    {}

    public function getMetadataBag(): MetadataBag
    {}

    public function registerBag(SessionBagInterface $bag): void
    {}

    public function getBag(string $name): SessionBagInterface
    {}

    /**
     * Gets the flashbag interface.
     */
    public function getFlashBag(): FlashBagInterface
    {}

    /**
     * Gets the attributebag interface.
     *
     * Note that this method was added to help with IDE autocompletion.
     */
    private function getAttributeBag(): AttributeBagInterface
    {}
}
