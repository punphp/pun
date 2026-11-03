<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\XmlConfiguration;

use function assert;
use function explode;
use function implode;
use DOMDocument;
use DOMElement;

/**
 * Whether dependencies between tests are resolved is not an ordering strategy.
 * The "depends" and "no-depends" values of the executionOrder attribute move to
 * the dedicated resolveDependencies attribute.
 *
 * The value of the resolveDependencies attribute is overwritten because the
 * tokens used to take precedence over it.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class MoveDependencyResolutionOutOfExecutionOrder implements Migration
{
    public function migrate(DOMDocument $document): void
    {}
}
