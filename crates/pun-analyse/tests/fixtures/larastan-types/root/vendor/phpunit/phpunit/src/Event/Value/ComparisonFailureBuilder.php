<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event\Code;

use function is_bool;
use function is_scalar;
use function print_r;
use PHPUnit\Framework\ExpectationFailedException;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ComparisonFailureBuilder
{
    public static function from(Throwable $t): ?ComparisonFailure
    {}

    private static function mapScalarValueToString(mixed $value): string
    {}
}
