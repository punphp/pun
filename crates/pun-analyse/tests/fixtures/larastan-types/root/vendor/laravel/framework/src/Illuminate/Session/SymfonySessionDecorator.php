<?php

namespace Illuminate\Session;

use BadMethodCallException;
use Illuminate\Contracts\Session\Session;
use Symfony\Component\HttpFoundation\Session\SessionBagInterface;
use Symfony\Component\HttpFoundation\Session\SessionInterface;
use Symfony\Component\HttpFoundation\Session\Storage\MetadataBag;

class SymfonySessionDecorator implements SessionInterface
{
    /**
     * The underlying Laravel session store.
     *
     * @var \Illuminate\Contracts\Session\Session
     */
    public readonly Session $store;

    /**
     * Create a new session decorator.
     *
     * @param  \Illuminate\Contracts\Session\Session  $store
     */
    public function __construct(Session $store)
    {}

    /**
     * {@inheritdoc}
     */
    public function start(): bool
    {}

    /**
     * {@inheritdoc}
     */
    public function getId(): string
    {}

    /**
     * {@inheritdoc}
     */
    public function setId(string $id): void
    {}

    /**
     * {@inheritdoc}
     */
    public function getName(): string
    {}

    /**
     * {@inheritdoc}
     */
    public function setName(string $name): void
    {}

    /**
     * {@inheritdoc}
     */
    public function invalidate(?int $lifetime = null): bool
    {}

    /**
     * {@inheritdoc}
     */
    public function migrate(bool $destroy = false, ?int $lifetime = null): bool
    {}

    /**
     * {@inheritdoc}
     */
    public function save(): void
    {}

    /**
     * {@inheritdoc}
     */
    public function has(string $name): bool
    {}

    /**
     * {@inheritdoc}
     */
    public function get(string $name, mixed $default = null): mixed
    {}

    /**
     * {@inheritdoc}
     */
    public function set(string $name, mixed $value): void
    {}

    /**
     * {@inheritdoc}
     */
    public function all(): array
    {}

    /**
     * {@inheritdoc}
     */
    public function replace(array $attributes): void
    {}

    /**
     * {@inheritdoc}
     */
    public function remove(string $name): mixed
    {}

    /**
     * {@inheritdoc}
     */
    public function clear(): void
    {}

    /**
     * {@inheritdoc}
     */
    public function isStarted(): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @throws \BadMethodCallException
     */
    public function registerBag(SessionBagInterface $bag): void
    {}

    /**
     * {@inheritdoc}
     *
     * @throws \BadMethodCallException
     */
    public function getBag(string $name): SessionBagInterface
    {}

    /**
     * {@inheritdoc}
     *
     * @throws \BadMethodCallException
     */
    public function getMetadataBag(): MetadataBag
    {}
}
