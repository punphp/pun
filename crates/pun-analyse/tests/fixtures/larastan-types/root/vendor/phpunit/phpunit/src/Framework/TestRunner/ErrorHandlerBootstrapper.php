<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\TestRunner;

use function array_reverse;
use function class_exists;
use function count;
use function explode;
use PHPUnit\Runner\Baseline\CannotLoadBaselineException;
use PHPUnit\Runner\Baseline\Reader;
use PHPUnit\Runner\DeprecationFilter;
use PHPUnit\Runner\ErrorHandler;
use PHPUnit\Runner\IssueTriggerResolver\Resolver;
use PHPUnit\TextUI\Configuration\Configuration;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ErrorHandlerBootstrapper
{
    public static function bootstrap(Configuration $configuration): void
    {}
}
