<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Configuration;

use const PHP_EOL;
use function in_array;
use function is_readable;
use function sprintf;
use PHPUnit\Event\Emitter;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class BootstrapLoader
{
    private readonly Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @throws BootstrapScriptDoesNotExistException
     * @throws BootstrapScriptException
     */
    public function handle(Configuration $configuration): void
    {}

    /**
     * @param non-empty-string $filename
     */
    private function load(string $filename): void
    {}
}
