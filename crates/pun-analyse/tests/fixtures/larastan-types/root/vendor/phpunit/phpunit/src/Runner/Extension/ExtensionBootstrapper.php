<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Extension;

use const PHP_EOL;
use function assert;
use function class_exists;
use function class_implements;
use function in_array;
use function sprintf;
use PHPUnit\Event\Emitter;
use PHPUnit\TextUI\Configuration\Configuration;
use ReflectionClass;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ExtensionBootstrapper
{
    private Configuration $configuration;
    private Facade $facade;
    private Emitter $emitter;

    public function __construct(Configuration $configuration, Facade $facade, Emitter $emitter)
    {}

    /**
     * @param non-empty-string      $className
     * @param array<string, string> $parameters
     */
    public function bootstrap(string $className, array $parameters): void
    {}
}
