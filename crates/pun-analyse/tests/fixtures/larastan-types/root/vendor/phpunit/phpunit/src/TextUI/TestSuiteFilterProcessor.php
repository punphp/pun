<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI;

use const FILE_IGNORE_NEW_LINES;
use const FILE_SKIP_EMPTY_LINES;
use function array_map;
use function file;
use PHPUnit\Event;
use PHPUnit\Framework\TestSuite;
use PHPUnit\Metadata\Api\Groups;
use PHPUnit\Runner\Filter\Factory;
use PHPUnit\TextUI\Configuration\Configuration;
use PHPUnit\TextUI\Configuration\FilterNotConfiguredException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestSuiteFilterProcessor
{
    private Event\Emitter $emitter;

    public function __construct(Event\Emitter $emitter)
    {}

    /**
     * @throws Event\RuntimeException
     * @throws FilterNotConfiguredException
     * @throws RuntimeException
     */
    public function process(Configuration $configuration, TestSuite $suite): void
    {}
}
