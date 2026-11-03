<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\EventDispatcher\Debug;

use Psr\EventDispatcher\StoppableEventInterface;
use Symfony\Component\EventDispatcher\EventDispatcherInterface;
use Symfony\Component\Stopwatch\Stopwatch;
use Symfony\Component\VarDumper\Caster\ClassStub;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
final class WrappedListener
{
    private string|array|object $listener;
    private ?\Closure $optimizedListener;
    private string $name;
    private bool $called = false;
    private bool $stoppedPropagation = false;
    private string $pretty;
    private string $callableRef;
    private ClassStub|string $stub;
    private static bool $hasClassStub;

    public function __construct(
        callable|array $listener,
        ?string $name,
        private Stopwatch $stopwatch,
        private ?EventDispatcherInterface $dispatcher = null,
        private ?int $priority = null,
    ) {}

    public function getWrappedListener(): callable|array
    {}

    public function wasCalled(): bool
    {}

    public function stoppedPropagation(): bool
    {}

    public function getPretty(): string
    {}

    public function getInfo(string $eventName): array
    {}

    public function __invoke(object $event, string $eventName, EventDispatcherInterface $dispatcher): void
    {}

    private function parseListener(array $listener): array
    {}
}
