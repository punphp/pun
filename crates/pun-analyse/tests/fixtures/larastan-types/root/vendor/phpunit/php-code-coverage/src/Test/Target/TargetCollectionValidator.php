<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Test\Target;

use function implode;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class TargetCollectionValidator
{
    public function validate(Mapper $mapper, TargetCollection $targets): ValidationResult
    {}
}
