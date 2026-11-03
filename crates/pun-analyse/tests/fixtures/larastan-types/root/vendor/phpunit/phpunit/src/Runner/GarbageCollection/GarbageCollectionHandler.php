<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\GarbageCollection;

use function gc_collect_cycles;
use function gc_disable;
use function gc_enable;
use PHPUnit\Event\Emitter;
use PHPUnit\Event\Facade;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class GarbageCollectionHandler
{
    private readonly Emitter $emitter;
    private readonly int $threshold;
    private int $tests = 0;

    public function __construct(Facade $facade, int $threshold)
    {}

    public function executionStarted(): void
    {}

    public function executionFinished(): void
    {}

    public function testFinished(): void
    {}

    private function registerSubscribers(Facade $facade): void
    {}
}
